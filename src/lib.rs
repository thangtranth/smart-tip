// To do
//  User stories:
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
//      - Click to each problem to tick and solve => Reward: increase point
//          - Click to complete a task -> call function task A complete -> increase point
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

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TaskId(pub u64);

// impl TaskId {
//     pub fn next(&self) -> Self {
//         Self(self.0 + 1)
//     }
// }

#[derive(BorshSerialize, BorshDeserialize)]
pub enum TaskStatus {
    PENDING,
    COMPLETE,
}
// pub struct Task {
//     task_id: TaskId,
//     task_status: TaskStatus
// }

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contributor {
    activity_point: U128,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Project {
    members: UnorderedMap<AccountId, Contributor>,
    amount_to_allocate: Balance,
    total_activity_point: U128,
    tasks: LookupMap<TaskId, TaskStatus>,
}

#[near_bindgen]
impl Project {
    // Initiate the contract
    #[init]
    pub fn new(member: Vec<AccountId>, amount_to_allocate: U128) -> Self {
        let mut project = Self {
            members: UnorderedMap::new(b"c".to_vec()),
            amount_to_allocate: amount_to_allocate.0,
            total_activity_point: 0.into(),
            tasks: LookupMap::new(b"s".to_vec()),
        };

        project
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

    // Add task
    pub fn add_task(&mut self, id: u64) {
        self.tasks.insert(&TaskId(id), &TaskStatus::PENDING);
    }
    // Update point when a task is completed.
    pub fn complete_activitiy(&mut self, task_id: u64) {
        self.tasks.insert(&TaskId(task_id), &TaskStatus::COMPLETE);
    }

    // Amount to allocate increase
    pub fn allocate_tip(&self, account_id: &AccountId) -> u128 {
        let contributor = self.members.get(account_id).unwrap();
        let allocation = (u128::from(self.total_activity_point)
            / u128::from(contributor.activity_point)
            * self.amount_to_allocate);

        env::log_str(&format!(
            "Contributor {} is allocated {} ",
            account_id, allocation
        ));
        allocation
    }

    // Send tips from smart contracts to near account.
    pub fn pay_tip(&self, account_id: AccountId) -> Promise {
        let tip_amount = self.allocate_tip(&account_id);
        Promise::new(account_id).transfer(tip_amount)
    }
}

mod test {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    #[test]
    fn tip_money() {
        // testing_env!(VMContextBuilder::new()
        //     .predecessor_account_id(accounts(0))
        //     .attached_deposit(10.into())
        //     .build());
        // let tip = Contract::pay(env::attached_deposit(), accounts(1));
    }
}
