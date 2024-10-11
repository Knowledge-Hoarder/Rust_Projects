use iced::widget::{ Button, Column, Container, Row, Scrollable, Text, TextInput };
use iced::{ Element, Length, Sandbox, Settings };
use iced::theme;
use iced::Color;
use crate::manager::RecipeManager;
use crate::recipe::Recipe;

pub struct RecipeManagerGUI {
    recipe_manager: RecipeManager,

    // Recipe form fields
    recipe_name: String,
    recipe_ingredients: String,
    recipe_instructions: String,
    recipe_servings: String,
    selected_recipe: Option<Recipe>,
    error_message: Option<String>,

    // Editable fields for recipe input
    edit_name: String,
    edit_ingredients: String,
    edit_instructions: String,
    edit_servings: String,
}

impl RecipeManagerGUI {
    fn view(&mut self) -> Element<'_, Message> {
        let content = Column::new()
            .padding(20)
            .spacing(10)
            .push(
                Text::new("Recipe Manager")
                    .size(40)
                    .color(Color::BLACK)
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
            )
            // Recipe Input Fields with Labels
            .push(Text::new("Recipe Name").size(20))
            .push(
                TextInput::new(
                    "Enter recipe name",
                    &self.recipe_name,
                    Message::NameChanged
                ).padding(10)
            )
            .push(Text::new("Ingredients").size(20))
            .push(
                TextInput::new(
                    "Enter ingredients",
                    &self.recipe_ingredients,
                    Message::IngredientsChanged
                ).padding(10)
            )
            .push(Text::new("Instructions").size(20))
            .push(
                TextInput::new(
                    "Enter instructions",
                    &self.recipe_instructions,
                    Message::InstructionsChanged
                ).padding(10)
            )
            .push(Text::new("Servings").size(20))
            .push(
                TextInput::new(
                    "Enter servings",
                    &self.recipe_servings,
                    Message::ServingsChanged
                ).padding(10)
            )

            // Add spacing and a Scrollable view for long recipes
            .push(
                Scrollable::new(Column::new().push(Text::new("Recipes List").size(30)).spacing(20))
            )

            // Add buttons for actions
            .push(
                Row::new()
                    .spacing(10)
                    .push(Button::new(Text::new("Add Recipe")).on_press(Message::AddRecipe))
                    .push(Button::new(Text::new("Clear Fields")).on_press(Message::ClearFields))
            )
            // Show error messages near inputs (if any)
            .push(
                if let Some(error) = &self.error_message {
                    Text::new(error.clone()).color(Color::from_rgb(1.0, 0.0, 0.0)) // red error message
                } else {
                    Text::new("")
                }
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}
