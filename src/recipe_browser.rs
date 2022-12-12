/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct RecipeBrowserApp {
    label: String,
    recipes: Vec<RecipeGuts>,
    selected_recipe : usize,
    ingredients_checked: Vec<bool>,
    methods_checked: Vec<bool>,
    recipe_is_selected: bool,

    // this how you opt-out of serialization of a member
    // #[serde(skip)]
    trace: bool,
}

impl Default for RecipeBrowserApp {
    fn default() -> Self {
        Self {
            label: "".to_owned(),
            recipes: vec![],
	    selected_recipe: 0,
	    ingredients_checked: vec![],
	    methods_checked: vec![],
	    recipe_is_selected: false,
            trace: false,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Default)]
#[derive(Debug)]
struct RecipeGuts {
    title: String,
    ingredients: Vec<String>,
    methods: Vec<String>,
}

impl RecipeGuts {
    pub fn new(title: String, ingredients: Vec<String>, methods: Vec<String>) -> Self {
        return RecipeGuts {
            title,
            ingredients,
            methods,
        };
    }
}

fn load_recipes( trace: bool ) -> serde_json::Value {
    use std::fs;

    let data = fs::read_to_string("/home/jncostanzo/git-repos/recipe-browser-eframe/recipe-browser-eframe/assets/recipes.json").expect("Unable to read file");
    if trace { println!("load_recipes: ../assets/recipes.json:\n{}", data); }

    let recipe_json: serde_json::Value =
        serde_json::from_str(&data).expect("JSON was not well-formatted");
    if trace { println!("load_recipes: Index={}\n", recipe_json["Index"]); }
    recipe_json
}

impl RecipeBrowserApp {

    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
	let trace = false;
        let rj: serde_json::Value = load_recipes( trace );
        if trace { println!("RecipeBrowserApp::new rj={}", rj); }

        let index = &rj["Index"];
        if trace { println!("RecipeBrowserApp::new rj[\"Index\"]={}", *index); }

        let recipe_count = (*index).as_array().unwrap().len();

        let mut recipes = Vec::new();

        for recipe_number in 0..recipe_count {
            if trace {
		println!(
                    "RecipeBrowserApp::new: recipe #{}='{}'",
                    recipe_number, index[recipe_number]
		);
	    }
            recipes.push(RecipeGuts::new(
                remove_leading_and_trailing_quotes( index[recipe_number]["Title"].to_string() ),
                serde_value_to_vec( index[recipe_number]["Ingredients"].clone(), trace ),
                serde_value_to_vec( index[recipe_number]["Method"].clone(), trace ),
            ));
        }
        if trace { println!("RecipeBrowserApp::new recipes={:#?}", recipes); }

        if trace { println!("RecipeBrowserApp::new recipes[1]={:#?}", recipes[1]); }

        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

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

fn serde_value_to_vec( v :serde_json::Value, trace: bool ) -> Vec<String> {
    let mut vec : Vec<String> = Vec::new();
    for i in v.as_array().iter() {
	for j in i.iter() {
	    if trace { println!("serdeValueToVec: {:#}", j ); }
	    vec.push( remove_leading_and_trailing_quotes( j.to_string() ) );
	}
    }
    vec
}

fn remove_leading_and_trailing_quotes( s: std::string::String ) -> std::string::String {
    // Given a String S, remove leading and trailing double quotes and return that string.
    use regex::Regex;
    let leading_quote_re = Regex::new("^\"").unwrap();
    let trailing_quote_re = Regex::new("\"$").unwrap();
    leading_quote_re.replace( &(trailing_quote_re.replace( s.as_str(), "") ), "" ).to_string()
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
	
        // let Self {
        //     label: _,
        //     recipes: _recipes,
	//     selected_recipe: _selected_recipe2,
	//     ingredients_checked: _ingredients_checked,
	//     methods_checked: _methods_checked,
	//     recipe_is_selected: _recipe_is_selected,
        //     trace: _trace2,
        // } = self;

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
                	let s : &String = &self.recipes[ n ].title.to_owned();
                	if ui.link(s).clicked() {

                	    recipe_text2 = s.to_owned();
			    self.selected_recipe = n;
			    self.ingredients_checked = vec![ false; self.recipes[ self.selected_recipe ].ingredients.len()];
			    self.methods_checked = vec![ false; self.recipes[ self.selected_recipe ].methods.len() ];
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
			let len = self.recipes[ self.selected_recipe ].ingredients.len();
			for i in 0..len {
			    let ingredient = RichText::new(self.recipes[ self.selected_recipe ].ingredients[ i ].to_owned());
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
			let len = self.recipes[ self.selected_recipe ].methods.len();
			for m in 0..len {
			    let method = RichText::new(self.recipes[ self.selected_recipe ].methods[ m ].to_owned() );
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
