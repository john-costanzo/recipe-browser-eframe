/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct RecipeBrowserApp {
    label: String,
    recipe_text : String,
    recipe_json : serde_json::Value,

    // this how you opt-out of serialization of a member
    // #[serde(skip)]
    trace : bool,
}

impl Default for RecipeBrowserApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            recipe_text: "<none>".to_owned(),
	    recipe_json: serde_json::json!(null),
	    trace : false,
        }
    }
}

fn load_recipes() -> serde_json::Value {
    use std::fs;

    let data = fs::read_to_string("/home/jncostanzo/git-repos/recipe-browser-eframe/recipe-browser-eframe/assets/recipes.json").expect("Unable to read file");
    println!("load_recipes: ../assets/recipes.json:\n{}", data );

    let recipe_json : serde_json::Value = serde_json::from_str(&data).expect("JSON was not well-formatted");
    println!("load_recipes: Index={}\n", recipe_json["Index"]);
    println!("load_recipes: Recipe #2={}\n", recipe_json["Index"][1]);
    println!("load_recipes: Recipe #3's Ingredients={}\n", recipe_json["Index"][2]["Text"]["Ingredients"]);

    recipe_json
}

impl RecipeBrowserApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {

	let rj : serde_json::Value = load_recipes();
	println!( "RecipeBrowserApp::new rj={}",rj);

	// TODO: hang onto recipe_titles
	// let recipe_titles =
	// for recipe_number in 0..*recipe_count {
	//     println!( "display_recipe_titles: recipe #{}='{}'", recipe_number, rj[recipe_number] );
	// }

	// TODO: hang onto recipe_texts
	// let recipe_texts = 

        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

	return Self {
	    recipe_json : rj,
            ..Default::default()
	};

        // // Load previous app state (if any).
        // // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // };

    }
}

fn display_recipe_titles( mut _ui : &egui::Ui, recipes : &serde_json::Value ) {
    println!("display_recipe_titles: recipes={}", recipes );
    let index = &recipes["Index"];
    let recipe_count : &usize = &recipes["Index"].as_array().unwrap().len();

    for x in 0..*recipe_count {
	println!( "display_recipe_titles: recipe #{}='{}'", x, index[x] );
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
        let Self { label : _, recipe_text : _recipe_text2, recipe_json : recipe_json2, trace : _trace2 } = self;

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

	    let _recipe_number: &'static str = "Recipe #";
	    println!("egui::Side_Panel: recipe_json2={}", recipe_json2);
	    // let recipe_count = recipe_json2["Index"].as_array().unwrap().len();
	    // println!("egui::Side_Panel: recipe_count={}", recipe_count);

	    egui::ScrollArea::new([false,true]).show(ui, |ui| {
		ui.vertical(|ui| {

		    // for n in 1..100 {
		    // 	let s : &str = &[ recipe_number, &n.to_string().to_owned() ].concat();
		    // 	if ui.link(s).clicked() {
		    // 	    println!( "egui::Side_Panel: {}", ["The '", s , "' link was clicked."].concat() );
		    // 	    *recipe_text2 = s.to_owned();
		    // 	}
		    // }
		    display_recipe_titles( ui, recipe_json2 );
		})
	    }
	    );
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
	    use eframe::egui::{Visuals};

            ui.heading("Recipe Browser");
            // let text = format!("This is the full text of the recipe:{}", recipe_text2);
            // ui.label(text);
	    ui.style_mut().visuals = Visuals::light();
            egui::warn_if_debug_build(ui);
        });
    }
}
