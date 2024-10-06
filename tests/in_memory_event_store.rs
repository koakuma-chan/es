use std::convert::Infallible;

use anyhow::Error;

use es::*;

#[derive(Default)]
struct Account {
    balance: i32,
}

enum Command {
    DepositMoney { amount: i32 },
}

#[derive(Clone)]
enum Event {
    MoneyDeposited { amount: i32 },
}

impl Handle for Account {
    type Command = Command;

    type Event = Event;

    type Error = Infallible;

    fn handle(&self, command: &Command) -> Result<Vec<Event>, Self::Error> {
        match *command {
            Command::DepositMoney { amount } => Ok(vec![Event::MoneyDeposited { amount }]),
        }
    }
}

impl Apply for Account {
    type Event = Event;

    fn apply(&mut self, event: &Event) {
        match event {
            Event::MoneyDeposited { amount } => {
                self.balance += amount;
            }
        }
    }
}

#[test]
fn it_works() -> Result<(), Error> {
    let mut event_store = InMemoryEventStore::default();

    let command = Command::DepositMoney { amount: 69 };

    <InMemoryEventStore<Event> as Execute<Account>>::execute(&mut event_store, 0, &command)?;

    let Account { balance } = event_store.project(0)?;

    assert_eq!(balance, 69);

    Ok(())
}
