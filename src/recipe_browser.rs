/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct RecipeBrowserApp {
    label: String,
    recipes: Vec<RecipeGuts>,
    recipe_json: serde_json::Value,
    selected_recipe : usize,

    // this how you opt-out of serialization of a member
    // #[serde(skip)]
    trace: bool,
}

impl Default for RecipeBrowserApp {
    fn default() -> Self {
        Self {
            label: "".to_owned(),
            recipes: vec![],
            recipe_json: serde_json::json!(null),
	    selected_recipe: 0,
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
    method: Vec<String>,
}

impl RecipeGuts {
    pub fn new(title: String, ingredients: Vec<String>, method: Vec<String>) -> Self {
        return RecipeGuts {
            title,
            ingredients,
            method,
        };
    }
}

fn load_recipes() -> serde_json::Value {
    use std::fs;

    let data = fs::read_to_string("/home/jncostanzo/git-repos/recipe-browser-eframe/recipe-browser-eframe/assets/recipes.json").expect("Unable to read file");
    println!("load_recipes: ../assets/recipes.json:\n{}", data);

    let recipe_json: serde_json::Value =
        serde_json::from_str(&data).expect("JSON was not well-formatted");
    println!("load_recipes: Index={}\n", recipe_json["Index"]);
    recipe_json
}

impl RecipeBrowserApp {

    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let rj: serde_json::Value = load_recipes();
        println!("RecipeBrowserApp::new rj={}", rj);

        let index = &rj["Index"];
        println!("RecipeBrowserApp::new rj[\"Index\"]={}", *index);

        let recipe_count = (*index).as_array().unwrap().len();

        let mut recipes = Vec::new();

        for recipe_number in 0..recipe_count {
            println!(
                "RecipeBrowserApp::new: recipe #{}='{}'",
                recipe_number, index[recipe_number]
            );
            recipes.push(RecipeGuts::new(
                remove_leading_and_trailing_quotes( index[recipe_number]["Title"].to_string() ),
                serde_value_to_vec( index[recipe_number]["Ingredients"].clone()),
                serde_value_to_vec( index[recipe_number]["Method"].clone()),
            ));
        }
        println!("RecipeBrowserApp::new recipes={:#?}", recipes);

        println!("RecipeBrowserApp::new recipes[1]={:#?}", recipes[1]);

        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Self {
            recipe_json: rj,
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

fn serde_value_to_vec( v :serde_json::Value ) -> Vec<String> {
    let mut vec : Vec<String> = Vec::new();
    for i in v.as_array().iter() {
	for j in i.iter() {
	    println!("serdeValueToVec: {:#}", j );
	    vec.push( remove_leading_and_trailing_quotes( j.to_string() ) );
	}
    }
    vec
}

fn format_recipe_text(ingredients:Vec<String>, methods: Vec<String>) -> std::string::String {
// Given INGREDIENTS and METHOD (which are both serde_json::Value as arrays) for a string containing them.
// TODO: format this nicer.
    let mut ingredients_text : String = String::from("");
    for ingredient in ingredients.iter() {
	println!("ingredient={:#}", ingredient );
	ingredients_text += ingredient;
    }

    let mut method_text : String = String::from("");
    for method in methods.iter() {
	println!("method={:#}", method );
	method_text += method;
    }

    println!( "format_recipe_text: returning [{}] [{}]", ingredients_text, method_text );
    format!("{}\n{}", ingredients_text, method_text)
}

fn remove_leading_and_trailing_quotes( s: std::string::String ) -> std::string::String {
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
	let mut recipe_text2 : String = "".to_string();
    
        let Self {
            label: _,
            recipes: _recipes,
            recipe_json: recipe_json2,
	    selected_recipe: _selected_recipe2,
            trace: _trace2,
        } = self;

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

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("\nRecipe Index");

            println!("egui::Side_Panel: recipe_json2={}", recipe_json2);
            let recipe_count = recipe_json2["Index"].as_array().unwrap().len();
            println!("egui::Side_Panel: recipe_count={}", recipe_count);

            egui::ScrollArea::new([false, true]).show(ui, |ui| {
                ui.vertical(|ui| {
                    for n in 0..recipe_count {
                	let s : &String = &self.recipes[ n ].title.to_owned();
                	if ui.link(s).clicked() {
                	    // println!( "egui::Side_Panel: {}", ["The '", &s , "' link was clicked."].concat() );
                	    recipe_text2 = s.to_owned();
			    self.selected_recipe = n;
                	}
                    }
                })
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            use eframe::egui::Visuals;

            ui.heading("Recipe Browser");
	    let selected_recipe_text : String = format_recipe_text(
		self.recipes[ self.selected_recipe ].ingredients.to_owned(),
		self.recipes[ self.selected_recipe ].method.to_owned());
            ui.label(selected_recipe_text);
            ui.style_mut().visuals = Visuals::light();
            egui::warn_if_debug_build(ui);
        });
    }
}
