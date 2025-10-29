//! ============================================
//! Frontend ↔ Backend Protocol
//! --------------------------------------------
//! Defines all signals, methods, and data types
//! exchanged between Flutter frontend and Rust
//! backend for the fitness tracker app.
//!
//! Author: Ishan Sharma
//! ============================================

use serde::{Deserialize, Serialize};
use chrono::NaiveTime;

/// ===============================
/// Methods the Frontend can CALL
/// ===============================
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
pub enum FrontendToBackend {
    /// Fetch summary for a specific date.
    ///
    /// **Parameters:**
    /// - `date`: String (format: "YYYY-MM-DD")
    ///
    /// **Returns:** `DaySummary`
    GetDaySummary {
        date: String,
    },

    /// Add a new meal for the last accessed date.
    ///
    /// **Parameters:**
    /// - `meal`: `Meal`
    ///
    /// **Returns:** `MealAdded`
    AddMeal {
        meal: Meal,
    },

    /// Add a new exercise for the last accessed date.
    ///
    /// **Parameters:**
    /// - `exercise`: `Exercise`
    ///
    /// **Returns:** `ExerciseAdded`
    AddExercise {
        exercise: Exercise,
    },
}

/// ===============================
/// Signals the Backend can SEND
/// ===============================
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "signal", content = "data")]
pub enum BackendToFrontend {
    /// Sent after a successful data fetch.
    ///
    /// **Data:** `DaySummary`
    DaySummary(DaySummary),

    /// Sent after successfully adding a meal.
    ///
    /// **Data:** `MealAdded`
    MealAdded(MealAdded),

    /// Sent after successfully adding an exercise.
    ///
    /// **Data:** `ExerciseAdded`
    ExerciseAdded(ExerciseAdded),

    /// Sent when an error occurs.
    ///
    /// **Data:** String (error message)
    Error(String),
}

/// ===============================
/// Core Data Types
/// ===============================

#[derive(Debug, Serialize, Deserialize)]
pub struct DaySummary {
    pub date: String,
    pub calories_consumed: i32,
    pub calories_burned: i32,
    pub target_consumed_calories: i32,
    pub target_burned_calories: i32,
    pub protein_consumed: i32,
    pub target_protein: i32,
    pub carbs_consumed: i32,
    pub target_carbs: i32,
    pub fat_consumed: i32,
    pub target_fat: i32,
    pub meals: Vec<Meal>,
    pub exercises: Vec<Exercise>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meal {
    pub total_calories: i32,
    pub time_finished: NaiveTime,
    pub foods: Vec<FoodItem>, // Each food item includes name + calories
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FoodItem {
    pub food_name: String,
    pub calories: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exercise {
    pub total_calories: i32,
    pub time_finished: NaiveTime,
    pub exercise_name: String,
    pub duration: i32, // in minutes or seconds
}

/// ===============================
/// Backend Response Structs
/// ===============================

#[derive(Debug, Serialize, Deserialize)]
pub struct MealAdded {
    pub date: String,
    pub meal: Meal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExerciseAdded {
    pub date: String,
    pub exercise: Exercise,
}
