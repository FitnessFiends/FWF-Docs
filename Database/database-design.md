## ER Diagram — Fitness App

The ER diagram represents a **fitness tracking application** designed to record user health, diet, and workout activities. It includes entities such as **Users**, **Meal**, **HealthInfo**, **DailyIntake**, **Workout**, and **Exercises**.

Each **User** must have an associated **HealthInfo** record, though some details (like activity level or birthday) may be missing.  
If a **User** logs a **Meal**, it contributes to their **DailyIntake**.  
When **Exercises** are recorded, they must belong to a **Workout**, but a **Workout** can exist without any exercises (for example, as a planned session).

---

### Entity Summary

| Entity          | Attributes                                                      | Primary Key             | Relationships                                                                                                              | Foreign Key(s)           |
| --------------- | --------------------------------------------------------------- | ----------------------- | -------------------------------------------------------------------------------------------------------------------------- | ------------------------ |
| **Users**       | uID, FName, LName, Username, Pronouns                           | uID                     | One-to-one with *HealthInfo*,<br>One-to-many with *Meal*,<br>One-to-many with *Workout*,<br>Many-to-one with *DailyIntake* | None                     |
| **HealthInfo**  | uID, Weight, Height, Gender, Birthday, Activity_Level           | uID                     | One-to-one with *Users*                                                                                                    | uID from *Users*         |
| **Meal**        | Carbs, Protein, Fats, Cals, MealTime, uID                       | Uid,MealTime            | Many-to-one with *Users*                                                                                                   | uID from *Users*         |
| **DailyIntake** | Carbs, Fats, Calories, Protein, Date, uID                       | uID, Date               | One-to-many with *Users*                                                                                                   | uID from *Users*         |
| **Workout**     | uID, WorkoutID, StartTime, EndTime, WorkoutName                 | WorkoutID               | Many-to-one with *Users*,<br>One-to-many with *Exercises*                                                                  | uID from *Users*         |
| **Exercises**   | ExerciseName, Weight, Sets, Reps, WorkoutID, StartTime, EndTime | WorkoutID, ExerciseName | Many-to-one with *Workout*                                                                                                 | WorkoutID from *Workout* |

