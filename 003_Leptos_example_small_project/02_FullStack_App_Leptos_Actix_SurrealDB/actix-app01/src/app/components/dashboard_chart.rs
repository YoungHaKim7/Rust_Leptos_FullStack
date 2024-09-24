use leptos::*;
use num_format::{Buffer, Locale};
use std::rc::Rc;

use crate::app::{components::DashboardWidget, models::Person};

#[component]
pub fn DashboardChart(persons_data: Vec<Person>) -> impl IntoView {
    // create a reference counting pointer to our actual persons data so
    // Rust doesn't need to create/clone copies of the actual data every time
    let retrieved_persons_data = Rc::new(persons_data.clone());

    // for counting the total number of team members
    let team_count = retrieved_persons_data.len().to_string();

    // for calculating and adding total cost for all the team members
    let mut total_cost = 0;

    // for identifying who is the latest to join
    let mut latest_member: String = String::new();
    let mut counter = 0;

    // loop through the returned data
    for person in persons_data {
        if counter == 0 {
            latest_member = person.name;
        }

        // adding this person's compensation to the total team cost
        total_cost += person.compensation;

        counter += 1;
    }

    // to convert the total cost to a string using the num-foramt crate's Buffer
    let mut buf = Buffer::default();
    buf.write_formatted(&total_cost, &Locale::en);
    let total_cost_str = format!("${}", buf.as_str());

    view! {
        <div class="w-full flex flex-col max-w-[64rem] mx-auto pt-8 mb-10">
            <div class="w-full h-20 grid grid-cols-3 gap-4 mx-auto px-2 max-w-[53rem]">
                <DashboardWidget title="Team Members" value=&team_count />
                <DashboardWidget title="Monthly Team Cost" value=&total_cost_str />
                <DashboardWidget title="Just Joined" value=&latest_member />
            </div>
        </div>
    }
    .into_view()
}
