use crate::schema::contributions;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Queryable, Debug, Insertable, PartialEq)]
#[table_name = "contributions"]
pub struct NewContribution {
    pub budget_id: i32,
    pub amount: i32,
    pub planned_date: NaiveDate,
    pub actual_date: Option<NaiveDate>,
}

#[derive(Queryable, Debug)]
pub struct Contribution {
    pub id: i32,
    pub budget_id: i32,
    pub amount: i32,
    pub planned_date: NaiveDate,
    pub actual_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::budget::test::mock_budget;
    use crate::models::test::*;
    use crate::schema::contributions::dsl::*;
    use diesel::prelude::*;

    impl From<Contribution> for NewContribution {
        fn from(input: Contribution) -> NewContribution {
            NewContribution {
                budget_id: input.budget_id,
                amount: input.amount,
                planned_date: input.planned_date,
                actual_date: input.actual_date,
            }
        }
    }

    #[test]
    fn test_contribution_db_round_trip() {
        run_in_transaction(&|conn| {
            let contribution = NewContribution {
                budget_id: mock_budget(conn)?,
                amount: 1000,
                planned_date: NaiveDate::from_ymd(1981, 8, 26),
                actual_date: None,
            };

            let committed_value = diesel::insert_into(contributions)
                .values(&contribution)
                .get_result::<Contribution>(conn);

            assert_eq!(NewContribution::from(committed_value?), contribution);

            Ok(())
        })
    }
}
