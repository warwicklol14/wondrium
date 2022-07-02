use yew::prelude::*;
use super::data_list_item_two_col::DataListItemTwoCol;
use super::list_header::ListHeader;
use super::data_list_item_single_col::DataListItemSingleCol;

#[derive(Properties, PartialEq)]
pub struct TwoColListProps {
}


#[function_component(TwoColList)]
pub fn two_col_list(props: &TwoColListProps) -> Html {
    html! {
        <>
        <div class="bg-white shadow overflow-hidden sm:rounded-lg">
            <div class="px-4 sm:px-6">
                <ListHeader title="Applicant Information" subtitle="Personal details and application"/>
            </div>
        <div class="border-t border-gray-200 px-4 py-5 sm:px-6">
            <dl class="grid grid-cols-1 gap-x-4 gap-y-8 sm:grid-cols-2">
                <DataListItemTwoCol label="Full name" value="Margot Foster"/>
                <DataListItemTwoCol label="Application for" value="Backend Developer"/>
                <DataListItemTwoCol label="Email Address" value="margotfoster@example.com"/>
                <DataListItemTwoCol label="Salary expectation" value="$120,000"/>
                <DataListItemSingleCol label="About" value="Fugiat ipsum ipsum deserunt culpa aute sint do nostrud anim incididunt cillum culpa consequat. Excepteur qui ipsum aliquip consequat sint. Sit id mollit nulla mollit nostrud in ea officia proident. Irure nostrud pariatur mollit ad adipisicing reprehenderit deserunt qui eu."/>
            </dl>
        </div>
        </div>
        </>
    }
}
