use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Debug)]
/// Represent a recipe.
pub struct Recipe {
    /// A recipe must have a title, 
    title: String,
    /// ...a list of ingredients and
    ingredients: Vec< String >,
    /// a list of methods.
    methods: Vec< String >
}

type RecipeIndex = HashMap< String, Vec<i32> >;
type RecipeDetails = Vec< Recipe >;

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Debug)]
/// Represent an indexed set of recipes.
pub struct Recipes {
    index: RecipeIndex,
    recipe_details: RecipeDetails
}

impl Recipes {
    /// Return a new (empty) recipe collection.
    ///
    /// No arguments.
    ///
    /// # Example
    ///
    /// ```
    /// // Create a new (empty) recipe collection.
    /// let recipe = Recipes::new();
    fn new() -> Self {
        Self {
            index: HashMap::new(),
            recipe_details: vec![],
        }
    }

    /// Return the number of recipes in this object.
    ///
    /// No arguments.
    ///
    /// # Example
    ///
    /// ```
    /// // Determine the number of recipes in a recipe collection.
    /// let recipe_count = recipes.len();
    pub fn len(&mut self) -> usize {
	self.recipe_details.len()
    }
}

// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

/// Represent the state of the Recipe Browser app.
pub struct RecipeBrowserApp {
    /// The recipes themselves, along with their index.
    recipes: Recipes,  
    /// The ordinal number of the selected recipe.
    selected_recipe : usize, 
    /// Which ingredients have been checked.
    ingredients_checked: Vec<bool>, 
    /// Which methods have been checked.
    methods_checked: Vec<bool>, 
    /// Has any recipe been selected?
    recipe_is_selected: bool,

    // this how you opt-out of serialization of a member
    // #[serde(skip)]
    /// Should we trace lifetime events dealing with this object?
    trace: bool,
}

impl Default for RecipeBrowserApp {
    /// A default contstructor.
    ///
    /// Return a new (empty) `RecipeBrowserApp`
    ///
    /// No arguments.
    ///
    /// # Example
    ///
    /// ```
    /// // Create a new (empty) RecipeBrowserApp.
    /// let app = RecipeBrowserApp:default();
    fn default() -> Self {
        Self {
            recipes: Recipes::new(),
	    selected_recipe: 0,
	    ingredients_checked: vec![],
	    methods_checked: vec![],
	    recipe_is_selected: false,
            trace: false,
        }
    }
}

    /// Load recipes into the application.
    ///
    /// GET the contents of a URL (expected to be in JSON), parse it and return an instance of `Recipes`.
    ///
    /// The function accepts 2 arguments:
    ///
    /// * `recipes_url`   The URL to use for GETting the JSON file.
    /// * `trace`         Whether to trace the running of this function.
    ///
    ///
    ///
    /// # Example
    ///
    /// ```
    /// // Load recipes
    /// let recipes = load_recipes( "https://some.site.com/recipes.json", false );
async fn load_recipes( recipes_url: String, trace: bool ) -> Result< Recipes, Box<dyn std::error::Error> > {
    if trace { println!("load_recipes: Starting..."); }
    let client = reqwest::Client::new();
    if trace { println!("load_recipes: About to get {}", recipes_url); }
    let data = client.get( recipes_url.to_owned() ).send()
        .await?
        .json::<Recipes>()
        .await?;
    
    if trace { println!("load_recipes: {}:\n{:#?}", recipes_url, data); }
    Ok( data )
}

impl RecipeBrowserApp {

    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
	let trace = false;
	if trace { println!("RecipeBrowserApp::new() starting..."); }

        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

	let mut recipes : Recipes = Recipes::new();
	let recipes_url :String = "https://raw.githubusercontent.com/john-costanzo/recipes/master/recipes.json".to_string(); // TODO: it would be nice if this were a const!
	if trace { println!("RecipeBrowserApp::new() about to call load_recipes; recipes={:#?}...",recipes); }

	let trt = tokio::runtime::Runtime::new().unwrap();

	let load_recipe_async_block = async {
	    match load_recipes( recipes_url, trace ).await {
		Ok(r) => {
		    recipes = r;
		    if trace { println!( "RecipeBrowserApp::new() back from load_recipes; recipes={:#?}...", recipes ); }
		},
		Err(e) => {
		    println!("load_recipes: Error loading file {:#?}", e );
		}
	    };
	};
	trt.block_on( load_recipe_async_block );

        Self {
	    recipes: recipes,
	    ..Default::default()
        }
	
        // // Load previous app state (if any).
        // // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // };
    }
}

impl eframe::App for RecipeBrowserApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
	use egui::{Color32,RichText};

	let mut recipe_text2 : String = "".to_string();
	

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("left_side_panel").show(ctx, |ui| {
            ui.heading("Index");

            let recipe_count = self.recipes.len();
            if self.trace { println!("egui::Side_Panel: recipe_count={}", recipe_count); }

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    for n in 0..recipe_count {
                	let s : &String = &self.recipes.recipe_details[ n ].title.to_owned();
                	if ui.link(s).clicked() {

                	    recipe_text2 = s.to_owned();
			    self.selected_recipe = n;
			    self.ingredients_checked = vec![ false; self.recipes.recipe_details[ self.selected_recipe ].ingredients.len()];
			    self.methods_checked = vec![ false; self.recipes.recipe_details[ self.selected_recipe ].methods.len() ];
			    self.recipe_is_selected = true;
                	}
                    }
                });
		ui.set_min_width(128.0);
            });
        });

        egui::SidePanel::right("right_side_panel").show(ctx, |ui| {
            ui.heading("Ingredients");

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
		    if self.recipe_is_selected {
			let len = self.recipes.recipe_details[ self.selected_recipe ].ingredients.len();
			for i in 0..len {
			    let ingredient = RichText::new(self.recipes.recipe_details[ self.selected_recipe ].ingredients[ i ].to_owned());
			    let color = if self.ingredients_checked[ i ] { Color32::GRAY } else { Color32::BLACK };
			    let mut rich_ingredients = ingredient.color(color);
			    if self.ingredients_checked[ i ] {
				rich_ingredients = rich_ingredients.italics();
			    }
			    ui.checkbox( &mut self.ingredients_checked[ i ], rich_ingredients );
			}
		    }
                });
		ui.set_min_width(160.0);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
	    ui.heading("Method");

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
		    if self.recipe_is_selected {
			let len = self.recipes.recipe_details[ self.selected_recipe ].methods.len();
			for m in 0..len {
			    let method = RichText::new(self.recipes.recipe_details[ self.selected_recipe ].methods[ m ].to_owned() );
			    let color = if self.methods_checked[ m ] { Color32::GRAY } else { Color32::BLACK };
			    let mut rich_method = method.color(color);
			    if self.methods_checked[ m ] {
				rich_method = rich_method.italics();
			    }
			    ui.checkbox( &mut self.methods_checked[ m ], rich_method );
			}
		    }
		});
		ui.set_min_width(320.0);
	    });
	    ctx.set_visuals(egui::Visuals::light());
	    //            egui::warn_if_debug_build(ui);
        });
    }
}
