use yew::prelude::*;

use wondrium_lib::Course;
use super::ui::CardContainer;
use super::ui::description_lists::ListHeader;
use super::ui::description_lists::DataListItem;
use super::ui::description_lists::DataListItemWithAction;
use super::ui::description_lists::ItemVariant;
use super::ui::layout::ListWithDividers;
use super::ui::layout::ListVariant;
use super::ui::layout::Divider;

#[derive(Properties, PartialEq)]
pub struct CourseContainerProps {
    pub course: Course,
    pub play_lecture: Callback<String>
}


#[function_component(CourseContainer)]
pub fn course_container(props: &CourseContainerProps) -> Html {
    let course = &props.course;
    let play_lecture = &props.play_lecture;
    html! {
        <>
            <CardContainer>
                <ListHeader title={ course.course_name.clone() } subtitle={format!("Course id: {}",course.course_id)}/>
                <ListWithDividers variant={ListVariant::TwoColList}>
                    <DataListItem variant={ItemVariant::SingleColWide} label="Course description" value={course.course_description.clone()} />
                </ListWithDividers>
                <Divider/>
                <ListWithDividers variant={ListVariant::VerticalList}>
                <ListHeader title="Lectures" subtitle={course.lectures.len().to_string()}/>
                {
                    for course.lectures.iter().map(|lecture| {
                    let onaction = {
                        let play_lecture = play_lecture.clone();
                        let lecture = lecture.clone();
                        Callback::from(move |_|{
                           play_lecture.emit(lecture.lecture_video_filename.clone());
                        })
                    };
                        html! {
                            <DataListItemWithAction label={lecture.lecture_name.clone()} value="" button_label="Play" {onaction}/>
                        }
                    })
                }
                </ListWithDividers>
            </CardContainer>
        </>
    }
}
