// To do
//  User stories:
// As the owner of the project, I could add tasks into the project so other people could contribute.
// As a contributor, I want to join the smarttip so that I could receive tip -> A project register to the smart contract -> Like create a DAO, each team will generate a project DAO
// As a contributor, I want to be added to the list of contributors after contributing so that I could receive the tip -> Add contributor after they complete a task
// As a contributor, I want to increase my score when I complete my contribution so that I could increase my share of tip -> Increase score of the contributor after
// As a users of the project, I want to tip the contributors so I could thank and motivate them
// As a conrtibutor, I want to receive tips basing on my contribution
// - Contract to receive tips, allocate the tips to each contributors
// When there is a tip
// - Contract: Project - Amount tips - Done
// - Receive a list of contributors with a token for that project - Done
//  - Project: the project that receive the tip -> Same as DAO - Done
//  - Accounts: individual contributors: activity point, accountID - Done
//  - Points: measuring activities -> Same as Berryclub - Done
// - Measure usages of individual
//      - Click to each problem to tick and solve => Reward: increase point - Done
//          - Check if the task id is exist or not?
//          - Click to complete a task -> call function task A complete -> increase point - Done
// - Allocate tips to each individual - Done
//  - Users call the smart contract to tips the project - Done

// Reference:
// tip contract
// collect point in frontend contract
// Berryclub contract: https://github.com/evgenykuzyakov/berryclub
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, Balance};
use near_sdk::{json_types::U128, near_bindgen, AccountId, Promise};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub struct TaskId(pub u64);

// impl TaskId {
//     pub fn next(&self) -> Self {
//         Self(self.0 + 1)
//     }
// }

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum TaskStatus {
    PENDING,
    COMPLETE,
}
// pub struct Task {
//     task_id: TaskId,
//     task_status: TaskStatus
// }

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Contribution {
    activity_point: U128,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Project {
    members: UnorderedMap<AccountId, Contribution>,
    amount_to_allocate: Balance,
    total_activity_point: U128,
    // Should it only inlude outstanding tasks?
    tasks: UnorderedMap<TaskId, TaskStatus>,
}

#[near_bindgen]
impl Project {
    // Initiate the contract
    #[init]
    pub fn new(member_list: Vec<AccountId>) -> Self {
        let mut project = Self {
            members: UnorderedMap::new(b"c".to_vec()),
            amount_to_allocate: 0,
            total_activity_point: 0.into(),
            tasks: UnorderedMap::new(b"s".to_vec()),
        };

        let initial_activity_point = Contribution {
            activity_point: 0.into(),
        };

        for member in member_list {
            project.members.insert(&member, &initial_activity_point);
        }

        project
    }

    // Add task
    pub fn add_task(&mut self, id: u64) {
        self.tasks.insert(&TaskId(id), &TaskStatus::PENDING);
    }

    // Update point when a task is completed.
    pub fn complete_activitiy(&mut self, task_id: u64) {
        assert!(
            self.tasks
                .keys_as_vector()
                .to_vec()
                .contains(&TaskId(task_id)),
            "The task is not existed"
        );
        let account_id = env::predecessor_account_id();
        self.tasks.insert(&TaskId(task_id), &TaskStatus::COMPLETE);
        let contribution = self.members.get(&account_id).unwrap_or(Contribution {
            activity_point: 0.into(),
        });

        let activity_point_update = contribution.activity_point.0 + 1;
        self.members.insert(
            &account_id,
            &Contribution {
                activity_point: (activity_point_update + 1).into(),
            },
        );
    }

    // Send Tip to the Smarttip
    pub fn tip(&mut self) -> u128 {
        let account_id = env::predecessor_account_id().to_string();
        let amount = env::attached_deposit();
        self.amount_to_allocate += amount;
        env::log_str(&format!(
            "Account {} tipped the project {}",
            account_id, amount
        ));

        amount
    }

    // Pay all contributors of the project after the tips.
    pub fn pay_all_contributors(&mut self) {
        for member in self.members.keys() {
            self.pay_tip(member);
        }

        self.amount_to_allocate = 0;
    }

    // Send tips from smart contracts to near account.
    pub fn pay_tip(&self, account_id: AccountId) -> Promise {
        let tip_amount = self.allocate_tip(&account_id);
        Promise::new(account_id).transfer(tip_amount)
    }

    // Amount to allocate increase
    pub fn allocate_tip(&self, account_id: &AccountId) -> u128 {
        let contributor = self.members.get(account_id).unwrap();
        let allocation = u128::from(contributor.activity_point)
            / u128::from(self.total_activity_point)
            * self.amount_to_allocate;

        env::log_str(&format!(
            "Contributor {} is allocated {} ",
            account_id, allocation
        ));
        allocation
    }
}

mod test {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    #[test]
    fn test_initiate() {
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(accounts(0))
            .attached_deposit(10)
            .build());
        let project = Project::new(vec![accounts(0), accounts(1)]);
        println!("Members of projects are {:#?}", project.members);
        assert_eq!(project.members.len(), 2);
        assert_eq!(project.amount_to_allocate, 0);
    }
    #[test]
    fn test_add_task() {
        let mut project = Project::new(vec![accounts(0)]);
        project.add_task(1);
        project.add_task(2);
        assert_eq!(project.tasks.len(), 2);
    }

    #[test]
    fn test_complete_task() {
        let mut project = Project::new(vec![accounts(0)]);
        project.add_task(1);
        project.complete_activitiy(1);
        println!(
            "Task status issss: {:#?}",
            project.tasks.get(&TaskId { 0: 1 }).unwrap(),
        );
        assert_eq!(
            project.tasks.get(&TaskId { 0: 1 }).unwrap(),
            TaskStatus::COMPLETE
        );
    }

    #[test]
    #[should_panic]
    fn test_non_existed_task() {
        let mut project = Project::new(vec![accounts(0)]);
        project.add_task(1);
        project.complete_activitiy(2);
        assert_eq!(
            project.tasks.get(&TaskId { 0: 1 }).unwrap(),
            TaskStatus::COMPLETE
        );
    }
    fn test_complete_activity() {}
    fn test_tip() {}
    fn test_allocate_tip() {}
    fn test_pay_tip() {}
    fn test_pay_all_contributors() {}
}
