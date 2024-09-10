use crate::models::course::*;
use crate::error::MyError;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_teacher_db
(pool: &PgPool, teacher_id: i32) -> Result<Vec<Course>, MyError> 
{
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course WHERE teacher_id = $1"#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_course_for_details_db
(pool: &PgPool, teacher_id: i32, course_id: i32) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course WHERE teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(course) = row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course didn't founded".into()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"INSERT INTO course (teacher_id, name, description, format, structure, duration, language, level, price)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, teacher_id, name, time, description, format, structure, duration, language, level, price"#,
        new_course.teacher_id,
        new_course.name,
        new_course.description,
        new_course.format,
        new_course.structure,
        new_course.duration,
        new_course.language,
        new_course.name,
        new_course.price,
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}