use yew::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::cell::Cell;
use crate::color::Color;

mod bindings;
mod cell;
mod color;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const RESOLUTIONS: [u32; 8] = [720, 900, 1080, 1280, 1440, 1600, 1920, 2048];
const DEFAULT_COLORS: [(u8, u8, u8); 3] = [
    (34, 80, 149),  // Blue
    (221, 1, 0),    // Red
    (250, 201, 1),  // Yellow
];

enum Msg {
    Generate,
    RenderImage,
    ToggleRandomColors,
    ToggleSaveCurrentColors,
    UpdateBorderColor(ChangeData),
    UpdateCanvasColor(ChangeData),
    UpdateCellFillChance(String),
    UpdateNumberOfColors(String),
    UpdateRatioX(String),
    UpdateRatioY(String),
    UpdateResolution(ChangeData),
}

struct Model {
    link: ComponentLink<Self>,
    rng: ThreadRng,
    max_width: u32,
    max_height: u32,
    ratio_x: u32,
    ratio_y: u32,
    resolution: u32,
    cell_fill_chance: u32,
    number_of_colors: u32,
    random_colors: bool,
    save_current_colors: bool,
    canvas_color: Color,
    border_color: Color,
    colors: Vec<(u8, u8, u8)>,
    cells: Vec<Cell>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let c = Self {
            link,
            rng: rand::thread_rng(),
            max_width: WIDTH,
            max_height: HEIGHT,
            ratio_x: 50,
            ratio_y: 50,
            resolution: RESOLUTIONS[0],
            cell_fill_chance: 30,
            number_of_colors: DEFAULT_COLORS.len() as u32,
            random_colors: false,
            save_current_colors: false,
            canvas_color: Color::new(255, 255, 255),
            border_color: Color::new(0, 0, 0),
            colors: DEFAULT_COLORS.clone().to_vec(),
            cells: Vec::new(),
        };
        c.link.send_message(Msg::Generate);
        c
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Generate => {
                if self.random_colors {
                    if !self.save_current_colors {
                        self.gen_random_colors();
                    }
                } else {
                    self.colors = DEFAULT_COLORS.clone().to_vec();
                }

                self.cells.clear();
                self.mondrian(&Cell::new(0, 0, self.max_width, self.max_height));
                self.link.send_message(Msg::RenderImage);
                true
            }
            Msg::RenderImage => {
                bindings::render_image(self.resolution);
                true
            }
            Msg::ToggleRandomColors => {
                self.random_colors = !self.random_colors;
                self.gen_random_colors();

                if !self.random_colors {
                    self.link.send_message(Msg::UpdateNumberOfColors(DEFAULT_COLORS.len().to_string()));
                }
                true
            }
            Msg::ToggleSaveCurrentColors => {
                self.save_current_colors = !self.save_current_colors;
                self.link.send_message(Msg::UpdateNumberOfColors(self.colors.len().to_string()));
                true
            }
            Msg::UpdateBorderColor(ChangeData::Value(border_color)) => {
                self.border_color.set_rgb(Color::hex_to_rgb(&border_color[1..]));
                true
            }
            Msg::UpdateCanvasColor(ChangeData::Value(canvas_color)) => {
                self.canvas_color.set_rgb(Color::hex_to_rgb(&canvas_color[1..]));
                true
            }
            Msg::UpdateCellFillChance(cell_fill_chance) => {
                self.cell_fill_chance = cell_fill_chance.parse().unwrap_or(30);
                true
            }
            Msg::UpdateNumberOfColors(number_of_colors) => {
                self.number_of_colors = number_of_colors.parse().unwrap_or(4);
                true
            }
            Msg::UpdateRatioX(ratio_x) => {
                self.ratio_x = ratio_x.parse().unwrap_or(20);
                true
            }
            Msg::UpdateRatioY(ratio_y) => {
                self.ratio_y = ratio_y.parse().unwrap_or(20);
                true
            }
            Msg::UpdateResolution(ChangeData::Select(resolution)) => {
                self.resolution = resolution.value().parse().unwrap_or(RESOLUTIONS[0]);
                true
            }
            _ => { false }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let container_style = format!("\
            position: absolute;
            width: {}px;
            left: 50%;
            margin-left: -{}px;
        ", self.max_width, self.max_width / 2);
        let controls_style = format!("\
            margin-top: {}px;
        ", self.max_height + 50);
        let canvas_color_value = format!("#{}", Color::rgb_to_hex(self.canvas_color.get_rgb()));
        let border_color_value = format!("#{}", Color::rgb_to_hex(self.border_color.get_rgb()));

        html! {
            <div style=container_style>
                <p>
                    { self.view_canvas() }
                </p>
                <div style=controls_style>
                    <p>
                        <button onclick=self.link.callback(|_| Msg::Generate)>{ "Generate" }</button>
                    </p>
                    <p style="width: 100%; display: flex; flex-direction: row; justify-content: center;
                              border-top: 1px solid darkgrey;">
                        <div class="card">
                            <h3>{ "Cells" }</h3>
                            <div class="ratio">
                                <label for="ratio-x">
                                    <span width="100px">{ "X Ratio" }</span>
                                    <span width="100px" style="margin-right: auto; float: right;">
                                        { self.ratio_x.to_string()}
                                    </span>
                                </label>
                                <input type="range" id="ratio-x" name="ratio-x" min="20" max="200" step="2"
                                    value=self.ratio_x.to_string()
                                    oninput=self.link.callback(|e: InputData| Msg::UpdateRatioX(e.value))/>
                            </div>
                            <div class="ratio">
                                <label for="ratio-y">
                                    <span width="100px">{ "Y Ratio" }</span>
                                    <span width="100px" style="margin-right: auto; float: right;">
                                        { self.ratio_y.to_string()}
                                    </span>
                                </label>
                                <input type="range" id="ratio-y" name="ratio-y" min="20" max="200" step="2"
                                    value=self.ratio_y.to_string()
                                    oninput=self.link.callback(|e: InputData| Msg::UpdateRatioY(e.value))/>
                            </div>
                            <div class="numParam">
                                <label>{ "Fill Chance (1-100):" }</label>
                                <div style="padding-left: 10px;">
                                    <input type="number" min="1" max="100"
                                        value=self.cell_fill_chance.to_string()
                                        oninput=self.link.callback(|e: InputData| Msg::UpdateCellFillChance(e.value))/>
                                </div>
                            </div>
                        </div>
                        <div class="card">
                            <div class="canvas">
                                <h3>{ "Canvas" }</h3>
                                <label>{ "Resolution:" }</label>
                                <select id="resolution" name="resolution"
                                    onchange=self.link.callback(|e: ChangeData| Msg::UpdateResolution(e))>
                                    {
                                        for RESOLUTIONS.iter().map(|res| html! {
                                            <option value=res.to_string()>
                                                { format!("{} x {}", res, res) }
                                            </option>
                                        })
                                    }
                                </select>
                            </div>
                        </div>
                        <div class="card">
                            <h3>{ "Colors" }</h3>
                            <div class="checkParam">
                                <label>
                                    <input type="checkbox" checked=self.random_colors
                                    onclick=self.link.callback(|_| Msg::ToggleRandomColors)/>
                                    <span style="padding-left: 10px;">{ "Random Colors" }</span>
                                </label>
                                <label>
                                    <input type="checkbox" checked=self.save_current_colors
                                    disabled={!self.random_colors}
                                    onclick=self.link.callback(|_| Msg::ToggleSaveCurrentColors)/>
                                    <span style="padding-left: 10px;">{ "Save Current Colors" }</span>
                                </label>
                            </div>
                            <div class="numParam">
                                <label>{ "Number of Colors (0-100):" }</label>
                                <div style="padding-left: 10px;">
                                    <input type="number" min="0" max="100"
                                        disabled={!self.random_colors || (self.random_colors && self.save_current_colors)}
                                        value=self.number_of_colors.to_string()
                                        oninput=self.link.callback(|e: InputData| Msg::UpdateNumberOfColors(e.value))/>
                                </div>
                            </div>
                            <div class="colorParam">
                                <label>
                                    <input type="color"
                                        value=canvas_color_value
                                        onchange=self.link.callback(|e: ChangeData| Msg::UpdateCanvasColor(e))/>
                                    <span style="padding-left: 10px;">{ "Canvas Color" }</span>
                                </label>
                            </div>
                            <div class="colorParam">
                                <label>
                                    <input type="color"
                                        value=border_color_value
                                        onchange=self.link.callback(|e: ChangeData| Msg::UpdateBorderColor(e))/>
                                    <span style="padding-left: 10px;">{ "Border Color" }</span>
                                </label>
                            </div>
                        </div>
                    </p>
                </div>
            </div>
        }
    }
}

impl Model {
    fn view_canvas(&self) -> Html {
        let cells = self.cells.iter().map(|cell| self.view_cell(cell));
        let image_style = format!("\
            width: {}px;
            height: {}px;
            position: absolute;
            z-index: 1;
        ", self.max_width, self.max_height);

        html! {
            <div>
                <svg class="art" width=self.max_width.to_string() height=self.max_height.to_string()
                    style="box-shadow: 0px 0px 10px grey; position: absolute; z-index: 0;">
                    { self.view_cell(&Cell::new(0, 0, self.max_width, self.max_height)) }
                    { for cells }
                </svg>
                <img class="image" style=image_style src="#" alt=""/>
            </div>
        }
    }

    fn view_cell(&self, cell: &Cell) -> Html {
        let style = format!("\
            fill: rgb{:?};
            stroke: rgb{:?};
            stroke-width: 3;
        ", cell.get_fill(), self.border_color.get_rgb());

        html! {
            <rect x=cell.x.to_string() y=cell.y.to_string() width=cell.width.to_string() height=cell.height.to_string()
                  style=style/>
        }
    }

    fn gen_random_colors(&mut self) {
        self.colors.clear();

        for _ in 0..self.number_of_colors {
            self.colors.push((
                self.rng.gen_range(0..=255),
                self.rng.gen_range(0..=255),
                self.rng.gen_range(0..=255),
            ));
        }
    }

    fn mondrian(&mut self, cell: &Cell) {
        let half_canvas_width = self.max_width / 2;
        let half_canvas_height = self.max_height / 2;

        let split_horizontally = |model: &mut Model| {
            let split_y = cell.y + model.rng.gen_range((cell.height / 4)..(cell.height - (cell.height / 4)));
            let (top, bottom) = cell.split_horizontally(split_y).unwrap();
            model.mondrian(&top);
            model.mondrian(&bottom);
        };
        let split_vertically = |model: &mut Model| {
            let split_x = cell.x + model.rng.gen_range((cell.width / 4)..(cell.width - (cell.width / 4)));
            let (left, right) = cell.split_vertically(split_x).unwrap();
            model.mondrian(&left);
            model.mondrian(&right);
        };
        let split_four_ways = |model: &mut Model| {
            let split_x = cell.x + model.rng.gen_range((cell.width / 4)..(cell.width - (cell.width / 4)));
            let split_y = cell.y + model.rng.gen_range((cell.height / 4)..(cell.height - (cell.height / 4)));
            let ((tl, tr), (bl, br)) = cell.split_four_ways(split_x, split_y).unwrap();
            model.mondrian(&tl);
            model.mondrian(&tr);
            model.mondrian(&bl);
            model.mondrian(&br);
        };

        if cell.width > half_canvas_width && cell.height > half_canvas_height {
            split_four_ways(self);
        } else if cell.width > half_canvas_width {
            split_vertically(self);
        } else if cell.height > half_canvas_height {
            split_horizontally(self);
        } else if cell.height > self.ratio_y && cell.width > self.ratio_x {
            split_four_ways(self);
        } else if cell.height > self.ratio_y {
            split_horizontally(self);
        } else if cell.width > self.ratio_x {
            split_vertically(self);
        } else {
            let mut cell = cell.clone();
            cell.set_fill(self.canvas_color.get_rgb());

            if self.number_of_colors > 0 {
                let n = self.rng.gen_range(0.0..100.0);
                let increment = self.cell_fill_chance as f32 / self.number_of_colors as f32;
                let mut i = 0;

                while i < self.number_of_colors {
                    if n < ((i + 1) as f32 * increment) {
                        cell.set_fill(*self.colors.get(i as usize).unwrap());
                        break;
                    }
                    i += 1;
                }
            }

            self.cells.push(cell);
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
