#![feature(ptr_offset_from, const_if_match, const_fn, const_panic, drain_filter)]
#![feature(const_generics)]

use crate::calc::{evaluate_tokens, CalcResult};
use crate::editor::{Editor, InputKey, InputModifiers};
use crate::renderer::render_result;
use crate::shunting_yard::ShuntingYard;
use crate::token_parser::{OperatorTokenType, Token, TokenParser, TokenType};
use crate::units::consts::{create_prefixes, init_units};
use crate::units::units::Units;
use crate::units::UnitPrefixes;
use smallvec::SmallVec;
use std::collections::HashMap;
use std::mem::MaybeUninit;

mod calc;
mod matrix;
mod shunting_yard;
mod token_parser;
mod units;

pub mod editor;
pub mod renderer;

const LINE_NUM_CONSTS: [[char; 3]; 256] = [
    [' ', ' ', '1'],
    [' ', ' ', '2'],
    [' ', ' ', '3'],
    [' ', ' ', '4'],
    [' ', ' ', '5'],
    [' ', ' ', '6'],
    [' ', ' ', '7'],
    [' ', ' ', '8'],
    [' ', ' ', '9'],
    [' ', '1', '0'],
    [' ', '1', '1'],
    [' ', '1', '2'],
    [' ', '1', '3'],
    [' ', '1', '4'],
    [' ', '1', '5'],
    [' ', '1', '6'],
    [' ', '1', '7'],
    [' ', '1', '8'],
    [' ', '1', '9'],
    [' ', '2', '0'],
    [' ', '2', '1'],
    [' ', '2', '2'],
    [' ', '2', '3'],
    [' ', '2', '4'],
    [' ', '2', '5'],
    [' ', '2', '6'],
    [' ', '2', '7'],
    [' ', '2', '8'],
    [' ', '2', '9'],
    [' ', '3', '0'],
    [' ', '3', '1'],
    [' ', '3', '2'],
    [' ', '3', '3'],
    [' ', '3', '4'],
    [' ', '3', '5'],
    [' ', '3', '6'],
    [' ', '3', '7'],
    [' ', '3', '8'],
    [' ', '3', '9'],
    [' ', '4', '0'],
    [' ', '4', '1'],
    [' ', '4', '2'],
    [' ', '4', '3'],
    [' ', '4', '4'],
    [' ', '4', '5'],
    [' ', '4', '6'],
    [' ', '4', '7'],
    [' ', '4', '8'],
    [' ', '4', '9'],
    [' ', '5', '0'],
    [' ', '5', '1'],
    [' ', '5', '2'],
    [' ', '5', '3'],
    [' ', '5', '4'],
    [' ', '5', '5'],
    [' ', '5', '6'],
    [' ', '5', '7'],
    [' ', '5', '8'],
    [' ', '5', '9'],
    [' ', '6', '0'],
    [' ', '6', '1'],
    [' ', '6', '2'],
    [' ', '6', '3'],
    [' ', '6', '4'],
    [' ', '6', '5'],
    [' ', '6', '6'],
    [' ', '6', '7'],
    [' ', '6', '8'],
    [' ', '6', '9'],
    [' ', '7', '0'],
    [' ', '7', '1'],
    [' ', '7', '2'],
    [' ', '7', '3'],
    [' ', '7', '4'],
    [' ', '7', '5'],
    [' ', '7', '6'],
    [' ', '7', '7'],
    [' ', '7', '8'],
    [' ', '7', '9'],
    [' ', '8', '0'],
    [' ', '8', '1'],
    [' ', '8', '2'],
    [' ', '8', '3'],
    [' ', '8', '4'],
    [' ', '8', '5'],
    [' ', '8', '6'],
    [' ', '8', '7'],
    [' ', '8', '8'],
    [' ', '8', '9'],
    [' ', '9', '0'],
    [' ', '9', '1'],
    [' ', '9', '2'],
    [' ', '9', '3'],
    [' ', '9', '4'],
    [' ', '9', '5'],
    [' ', '9', '6'],
    [' ', '9', '7'],
    [' ', '9', '8'],
    [' ', '9', '9'],
    ['1', '0', '0'],
    ['1', '0', '1'],
    ['1', '0', '2'],
    ['1', '0', '3'],
    ['1', '0', '4'],
    ['1', '0', '5'],
    ['1', '0', '6'],
    ['1', '0', '7'],
    ['1', '0', '8'],
    ['1', '0', '9'],
    ['1', '1', '0'],
    ['1', '1', '1'],
    ['1', '1', '2'],
    ['1', '1', '3'],
    ['1', '1', '4'],
    ['1', '1', '5'],
    ['1', '1', '6'],
    ['1', '1', '7'],
    ['1', '1', '8'],
    ['1', '1', '9'],
    ['1', '2', '0'],
    ['1', '2', '1'],
    ['1', '2', '2'],
    ['1', '2', '3'],
    ['1', '2', '4'],
    ['1', '2', '5'],
    ['1', '2', '6'],
    ['1', '2', '7'],
    ['1', '2', '8'],
    ['1', '2', '9'],
    ['1', '3', '0'],
    ['1', '3', '1'],
    ['1', '3', '2'],
    ['1', '3', '3'],
    ['1', '3', '4'],
    ['1', '3', '5'],
    ['1', '3', '6'],
    ['1', '3', '7'],
    ['1', '3', '8'],
    ['1', '3', '9'],
    ['1', '4', '0'],
    ['1', '4', '1'],
    ['1', '4', '2'],
    ['1', '4', '3'],
    ['1', '4', '4'],
    ['1', '4', '5'],
    ['1', '4', '6'],
    ['1', '4', '7'],
    ['1', '4', '8'],
    ['1', '4', '9'],
    ['1', '5', '0'],
    ['1', '5', '1'],
    ['1', '5', '2'],
    ['1', '5', '3'],
    ['1', '5', '4'],
    ['1', '5', '5'],
    ['1', '5', '6'],
    ['1', '5', '7'],
    ['1', '5', '8'],
    ['1', '5', '9'],
    ['1', '6', '0'],
    ['1', '6', '1'],
    ['1', '6', '2'],
    ['1', '6', '3'],
    ['1', '6', '4'],
    ['1', '6', '5'],
    ['1', '6', '6'],
    ['1', '6', '7'],
    ['1', '6', '8'],
    ['1', '6', '9'],
    ['1', '7', '0'],
    ['1', '7', '1'],
    ['1', '7', '2'],
    ['1', '7', '3'],
    ['1', '7', '4'],
    ['1', '7', '5'],
    ['1', '7', '6'],
    ['1', '7', '7'],
    ['1', '7', '8'],
    ['1', '7', '9'],
    ['1', '8', '0'],
    ['1', '8', '1'],
    ['1', '8', '2'],
    ['1', '8', '3'],
    ['1', '8', '4'],
    ['1', '8', '5'],
    ['1', '8', '6'],
    ['1', '8', '7'],
    ['1', '8', '8'],
    ['1', '8', '9'],
    ['1', '9', '0'],
    ['1', '9', '1'],
    ['1', '9', '2'],
    ['1', '9', '3'],
    ['1', '9', '4'],
    ['1', '9', '5'],
    ['1', '9', '6'],
    ['1', '9', '7'],
    ['1', '9', '8'],
    ['1', '9', '9'],
    ['2', '0', '0'],
    ['2', '0', '1'],
    ['2', '0', '2'],
    ['2', '0', '3'],
    ['2', '0', '4'],
    ['2', '0', '5'],
    ['2', '0', '6'],
    ['2', '0', '7'],
    ['2', '0', '8'],
    ['2', '0', '9'],
    ['2', '1', '0'],
    ['2', '1', '1'],
    ['2', '1', '2'],
    ['2', '1', '3'],
    ['2', '1', '4'],
    ['2', '1', '5'],
    ['2', '1', '6'],
    ['2', '1', '7'],
    ['2', '1', '8'],
    ['2', '1', '9'],
    ['2', '2', '0'],
    ['2', '2', '1'],
    ['2', '2', '2'],
    ['2', '2', '3'],
    ['2', '2', '4'],
    ['2', '2', '5'],
    ['2', '2', '6'],
    ['2', '2', '7'],
    ['2', '2', '8'],
    ['2', '2', '9'],
    ['2', '3', '0'],
    ['2', '3', '1'],
    ['2', '3', '2'],
    ['2', '3', '3'],
    ['2', '3', '4'],
    ['2', '3', '5'],
    ['2', '3', '6'],
    ['2', '3', '7'],
    ['2', '3', '8'],
    ['2', '3', '9'],
    ['2', '4', '0'],
    ['2', '4', '1'],
    ['2', '4', '2'],
    ['2', '4', '3'],
    ['2', '4', '4'],
    ['2', '4', '5'],
    ['2', '4', '6'],
    ['2', '4', '7'],
    ['2', '4', '8'],
    ['2', '4', '9'],
    ['2', '5', '0'],
    ['2', '5', '1'],
    ['2', '5', '2'],
    ['2', '5', '3'],
    ['2', '5', '4'],
    ['2', '5', '5'],
    ['2', '5', '6'],
];

const MAX_EDITOR_WIDTH: usize = 120;
const LEFT_GUTTER_WIDTH: usize = 1 + 3 + 1;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TextStyle {
    Normal,
    Bold,
    Underline,
    Italy,
}

#[derive(Debug)]
pub struct RenderTextMsg<'a> {
    pub text: &'a [char],
    pub row: usize,
    pub column: usize,
}

#[repr(C)]
#[derive(Debug)]
pub enum OutputMessage<'a> {
    SetStyle(TextStyle),
    SetColor(u32),
    RenderChar(usize, usize, char),
    RenderText(RenderTextMsg<'a>),
    RenderRectangle {
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    },
}

#[derive(Debug)]
pub struct RenderBuckets<'a> {
    pub texts: Vec<RenderTextMsg<'a>>,
    pub numbers: Vec<RenderTextMsg<'a>>,
    pub units: Vec<RenderTextMsg<'a>>,
    pub operators: Vec<RenderTextMsg<'a>>,
    pub variable: Vec<RenderTextMsg<'a>>,
    pub custom_commands: [Vec<OutputMessage<'a>>; 2],
}

#[repr(C)]
pub enum Layer {
    BehindText,
    AboveText,
}

impl<'a> RenderBuckets<'a> {
    pub fn new() -> RenderBuckets<'a> {
        RenderBuckets {
            texts: Vec::with_capacity(128),
            custom_commands: [Vec::with_capacity(128), Vec::with_capacity(128)],
            numbers: Vec::with_capacity(32),
            units: Vec::with_capacity(32),
            operators: Vec::with_capacity(32),
            variable: Vec::with_capacity(32),
        }
    }

    pub fn set_color(&mut self, layer: Layer, color: u32) {
        self.custom_commands[layer as usize].push(OutputMessage::SetColor(color));
    }

    pub fn draw_rect(&mut self, layer: Layer, x: usize, y: usize, w: usize, h: usize) {
        self.custom_commands[layer as usize].push(OutputMessage::RenderRectangle { x, y, w, h });
    }

    pub fn draw_char(&mut self, layer: Layer, x: usize, y: usize, ch: char) {
        self.custom_commands[layer as usize].push(OutputMessage::RenderChar(x, y, ch));
    }

    pub fn draw_text(&mut self, layer: Layer, x: usize, y: usize, text: &'static [char]) {
        self.custom_commands[layer as usize].push(OutputMessage::RenderText(RenderTextMsg {
            text,
            row: y,
            column: x,
        }));
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ResultFormat {
    Bin,
    Dec,
    Hex,
}

pub struct LineData {
    result_format: ResultFormat,
}

impl Default for LineData {
    fn default() -> Self {
        LineData {
            result_format: ResultFormat::Dec,
        }
    }
}

pub struct NoteCalcApp<'a> {
    client_width: usize,
    units: Units<'a>,
    pub editor: Editor,
    line_datas: Vec<LineData>,
    prefixes: &'static UnitPrefixes,
    result_buffer: [char; 1024],
}

impl<'a> NoteCalcApp<'a> {
    pub fn new(client_width: usize) -> NoteCalcApp<'a> {
        let prefixes: &'static UnitPrefixes = Box::leak(Box::new(create_prefixes()));
        let units = Units::new(&prefixes);
        let mut line_datas = Vec::with_capacity(32);
        NoteCalcApp {
            client_width,
            prefixes,
            units,
            editor: Editor::new(MAX_EDITOR_WIDTH, &mut line_datas),
            line_datas,
            result_buffer: [0 as char; 1024],
        }
    }

    pub fn set_content(&mut self, text: &str) {
        return self.editor.set_content(text, &mut self.line_datas);
    }

    pub fn render(&mut self) -> RenderBuckets {
        let RIGHT_GUTTER_WIDTH = 3;
        let MIN_RESULT_PANEL_WIDTH = 20;
        let result_gutter_x = (LEFT_GUTTER_WIDTH + MAX_EDITOR_WIDTH)
            .min(self.client_width - (RIGHT_GUTTER_WIDTH + MIN_RESULT_PANEL_WIDTH));
        let current_editor_width = result_gutter_x - LEFT_GUTTER_WIDTH;

        // TODO: improve vec alloc
        let mut render_buckets = RenderBuckets::new();
        let mut result_buffer_index = 0;
        let mut result_str_positions: SmallVec<[Option<(usize, usize)>; 256]> =
            SmallVec::with_capacity(256);
        let mut longest_row_len = 0;

        // result gutter
        render_buckets.set_color(Layer::BehindText, 0xF2F2F2_FF);
        render_buckets.draw_rect(
            Layer::BehindText,
            result_gutter_x,
            0,
            RIGHT_GUTTER_WIDTH,
            255,
        );

        let mut vars: Vec<(&[char], CalcResult)> = Vec::new();
        let mut render_y = 0;
        for (row_index, line) in self.editor.lines().enumerate() {
            if line.len() > longest_row_len {
                longest_row_len = line.len();
            }

            // TODO optimize vec allocations
            let mut tokens = Vec::with_capacity(128);
            TokenParser::parse_line(line, &vars, &[], &mut tokens, &self.units);

            let mut shunting_output_stack = Vec::with_capacity(128);
            ShuntingYard::shunting_yard(&mut tokens, &[], &mut shunting_output_stack);

            // render
            let mut render_x = 0;
            let rendered_row_height = NoteCalcApp::calc_rendered_row_height(&tokens);
            let mut token_index = 0;
            while token_index < tokens.len() {
                let token = &tokens[token_index];
                let need_matrix_render = self.line_datas[row_index].result_format
                    == ResultFormat::Hex
                    && rendered_row_height > 1;
                if let (
                    TokenType::Operator(OperatorTokenType::Matrix {
                        row_count,
                        col_count,
                    }),
                    true,
                ) = (&token.typ, need_matrix_render)
                {
                    render_x = NoteCalcApp::render_matrix(
                        render_x,
                        render_y,
                        current_editor_width,
                        *row_count,
                        *col_count,
                        &tokens[token_index..],
                        &mut render_buckets,
                    );

                    while tokens[token_index].typ
                        != TokenType::Operator(OperatorTokenType::BracketClose)
                    {
                        token_index += 1;
                    }
                    // ignore the brakcet as well
                    token_index += 1;
                } else {
                    NoteCalcApp::draw_token(
                        token,
                        render_x,
                        render_y,
                        current_editor_width,
                        &mut render_buckets,
                    );
                    token_index += 1;
                    render_x += token.ptr.len();
                }
            }
            if render_x > current_editor_width {
                render_buckets.draw_char(
                    Layer::AboveText,
                    current_editor_width + LEFT_GUTTER_WIDTH,
                    render_y,
                    '…',
                );
            }

            if let Some(result) = evaluate_tokens(&mut shunting_output_stack, &self.units, &vars) {
                let result_str = render_result(
                    &self.units,
                    &result.result,
                    &self.line_datas[row_index].result_format,
                    result.there_was_unit_conversion,
                );

                let start = result_buffer_index;
                for ch in result_str.chars() {
                    self.result_buffer[result_buffer_index] = ch;
                    result_buffer_index += 1;
                }
                result_str_positions.push(Some((start, result_buffer_index)));
                if result.assignment {
                    let var_name = {
                        let mut i = 0;
                        // skip whitespaces
                        while line[i].is_ascii_whitespace() {
                            i += 1;
                        }
                        let start = i;
                        // take until '='
                        while line[i] != '=' {
                            i += 1;
                        }
                        // remove trailing whitespaces
                        i -= 1;
                        while line[i].is_ascii_whitespace() {
                            i -= 1;
                        }
                        let end = i;
                        &line[start..=end]
                    };
                    vars.push((var_name, result.result))
                }
            } else {
                result_str_positions.push(None);
            };

            match self.line_datas[row_index].result_format {
                ResultFormat::Hex => {
                    render_buckets.operators.push(RenderTextMsg {
                        text: &['0', 'x'],
                        row: render_y,
                        column: result_gutter_x + 1,
                    });
                }
                ResultFormat::Bin => {
                    render_buckets.operators.push(RenderTextMsg {
                        text: &['0', 'b'],
                        row: render_y,
                        column: result_gutter_x + 1,
                    });
                }
                ResultFormat::Dec => {}
            }
            render_y += rendered_row_height;
        }
        for (row_i, pos) in result_str_positions.iter().enumerate() {
            if let Some((start, end)) = pos {
                render_buckets.texts.push(RenderTextMsg {
                    text: &self.result_buffer[*start..*end],
                    row: row_i,
                    column: result_gutter_x + RIGHT_GUTTER_WIDTH,
                });
            }
        }

        let cursor_pos = self.editor.get_selection().get_cursor_pos();
        if self.editor.show_cursor && cursor_pos.column < current_editor_width {
            render_buckets.texts.push(RenderTextMsg {
                text: &['▏'],
                row: cursor_pos.row,
                column: cursor_pos.column + LEFT_GUTTER_WIDTH,
            });
        }

        // gutter
        render_buckets.set_color(Layer::BehindText, 0xF2F2F2_FF);
        render_buckets.draw_rect(Layer::BehindText, 0, 0, LEFT_GUTTER_WIDTH, 255);

        // highlight current line
        render_buckets.set_color(Layer::BehindText, 0xFCFAED_C8);
        render_buckets.draw_rect(
            Layer::BehindText,
            0,
            self.editor.get_selection().get_cursor_pos().row,
            result_gutter_x + RIGHT_GUTTER_WIDTH + MIN_RESULT_PANEL_WIDTH,
            1,
        );
        // line numbers
        render_buckets.set_color(Layer::BehindText, 0xADADAD_FF);
        for i in 0..255 {
            render_buckets.custom_commands[Layer::BehindText as usize].push(
                OutputMessage::RenderText(RenderTextMsg {
                    text: &(LINE_NUM_CONSTS[i][..]),
                    row: i,
                    column: 1,
                }),
            )
        }

        // selected text
        render_buckets.set_color(Layer::BehindText, 0xA6D2FF_FF);
        if self.editor.get_selection().is_range() {
            let start = self.editor.get_selection().get_first();
            let end = self.editor.get_selection().get_second();
            if end.row > start.row {
                // first line
                render_buckets.draw_rect(
                    Layer::BehindText,
                    start.column + LEFT_GUTTER_WIDTH,
                    start.row,
                    (MAX_EDITOR_WIDTH - start.column).min(current_editor_width),
                    1,
                );
                // full lines
                let height = end.row - start.row - 1;
                render_buckets.draw_rect(
                    Layer::BehindText,
                    LEFT_GUTTER_WIDTH,
                    start.row + 1,
                    current_editor_width,
                    height,
                );
                // last line
                render_buckets.draw_rect(
                    Layer::BehindText,
                    LEFT_GUTTER_WIDTH,
                    end.row,
                    end.column.min(current_editor_width),
                    1,
                );
            } else {
                render_buckets.draw_rect(
                    Layer::BehindText,
                    start.column + LEFT_GUTTER_WIDTH,
                    start.row,
                    (end.column - start.column).min(current_editor_width),
                    1,
                );
            }
        }

        return render_buckets;
    }

    fn render_matrix<'b>(
        mut render_x: usize,
        mut render_y: usize,
        current_editor_width: usize,
        row_count: usize,
        col_count: usize,
        tokens: &[Token<'b, 'b>],
        render_buckets: &mut RenderBuckets<'b>,
    ) -> usize {
        render_buckets.operators.push(RenderTextMsg {
            text: &['⎡'],
            row: render_y,
            column: render_x + LEFT_GUTTER_WIDTH,
        });
        for i in 1..row_count - 1 {
            render_buckets.operators.push(RenderTextMsg {
                text: &['⎢'],
                row: render_y + i,
                column: render_x + LEFT_GUTTER_WIDTH,
            });
        }
        render_buckets.operators.push(RenderTextMsg {
            text: &['⎣'],
            row: render_y + row_count - 1,
            column: render_x + LEFT_GUTTER_WIDTH,
        });
        render_x += 1;

        let mut tokens_per_cell = {
            let mut tokens_per_cell: [MaybeUninit<&[Token]>; 32] =
                unsafe { MaybeUninit::uninit().assume_init() };

            let mut start_token_index = 0;
            let mut cell_index = 0;
            let mut can_ignore_ws = true;
            for (token_index, token) in tokens.iter().enumerate() {
                if token.typ == TokenType::Operator(OperatorTokenType::BracketClose) {
                    tokens_per_cell[cell_index] =
                        MaybeUninit::new(&tokens[start_token_index..token_index]);
                    break;
                } else if token.typ
                    == TokenType::Operator(OperatorTokenType::Matrix {
                        row_count,
                        col_count,
                    })
                    || token.typ == TokenType::Operator(OperatorTokenType::BracketOpen)
                {
                    // skip them
                    start_token_index = token_index + 1;
                } else if can_ignore_ws && token.ptr[0].is_ascii_whitespace() {
                    start_token_index = token_index + 1;
                } else if token.typ == TokenType::Operator(OperatorTokenType::Comma)
                    || token.typ == TokenType::Operator(OperatorTokenType::Semicolon)
                {
                    tokens_per_cell[cell_index] =
                        MaybeUninit::new(&tokens[start_token_index..token_index]);
                    start_token_index = token_index + 1;
                    cell_index += 1;
                    can_ignore_ws = true;
                } else {
                    can_ignore_ws = false;
                }
            }
            unsafe { std::mem::transmute::<_, [&[Token]; 32]>(tokens_per_cell) }
        };

        for col_i in 0..col_count {
            if render_x >= current_editor_width {
                return render_x;
            }
            let max_width: usize = (0..row_count)
                .map(|row_i| {
                    tokens_per_cell[row_i * col_count + col_i]
                        .iter()
                        .map(|it| it.ptr.len())
                        .sum()
                })
                .max()
                .unwrap();
            for row_i in 0..row_count {
                let tokens = &tokens_per_cell[row_i * col_count + col_i];
                let len: usize = tokens.iter().map(|it| it.ptr.len()).sum();
                let offset_x = max_width - len;
                let mut local_x = 0;
                for token in tokens.iter() {
                    NoteCalcApp::draw_token(
                        token,
                        render_x + offset_x + local_x,
                        render_y + row_i,
                        current_editor_width,
                        render_buckets,
                    );
                    local_x += token.ptr.len();
                }
            }
            render_x += if col_i + 1 < col_count {
                max_width + 2
            } else {
                max_width
            };
        }

        render_buckets.operators.push(RenderTextMsg {
            text: &['⎤'],
            row: render_y,
            column: render_x + LEFT_GUTTER_WIDTH,
        });
        for i in 1..row_count - 1 {
            render_buckets.operators.push(RenderTextMsg {
                text: &['⎥'],
                row: render_y + i,
                column: render_x + LEFT_GUTTER_WIDTH,
            });
        }
        render_buckets.operators.push(RenderTextMsg {
            text: &['⎦'],
            row: render_y + row_count - 1,
            column: render_x + LEFT_GUTTER_WIDTH,
        });
        render_x += 1;

        render_x
    }

    fn draw_token<'b>(
        token: &Token<'b, 'b>,
        render_x: usize,
        render_y: usize,
        current_editor_width: usize,
        render_buckets: &mut RenderBuckets<'b>,
    ) {
        let dst = match &token.typ {
            TokenType::StringLiteral => &mut render_buckets.texts,
            TokenType::Variable(_) => &mut render_buckets.variable,
            TokenType::NumberLiteral(_) => &mut render_buckets.numbers,
            TokenType::Operator(op_type) => match op_type {
                OperatorTokenType::Unit(_) => &mut render_buckets.units,
                _ => &mut render_buckets.operators,
            },
        };
        let text_len = token
            .ptr
            .len()
            .min((current_editor_width as isize - render_x as isize).max(0) as usize);
        dst.push(RenderTextMsg {
            text: &token.ptr[0..text_len],
            row: render_y,
            column: render_x + LEFT_GUTTER_WIDTH,
        });
    }

    fn calc_rendered_row_height(tokens: &[Token]) -> usize {
        let mut max_height = 1;
        for token in tokens {
            match token.typ {
                TokenType::Operator(OperatorTokenType::Matrix {
                    row_count,
                    col_count,
                }) => {
                    if row_count > max_height {
                        max_height = row_count;
                    }
                }
                _ => {}
            }
        }
        return max_height;
    }

    pub fn handle_click(&mut self, x: usize, y: usize) {
        let editor = &mut self.editor;
        if x < LEFT_GUTTER_WIDTH {
            // clicked on gutter
        } else if x - LEFT_GUTTER_WIDTH < MAX_EDITOR_WIDTH {
            editor.handle_click(x - LEFT_GUTTER_WIDTH, y);
        }
    }

    pub fn handle_resize(&mut self, new_client_width: usize) {
        self.client_width = new_client_width;
    }

    pub fn handle_drag(&mut self, x: usize, y: usize) {
        let editor = &mut self.editor;
        if x < LEFT_GUTTER_WIDTH {
            // clicked on gutter
        } else if x - LEFT_GUTTER_WIDTH < MAX_EDITOR_WIDTH {
            editor.handle_drag(x - LEFT_GUTTER_WIDTH, y);
        }
    }

    pub fn handle_input(&mut self, input: InputKey, modifiers: InputModifiers) {
        if modifiers.alt && input == InputKey::Left {
            let cur_pos = self.editor.get_selection().get_cursor_pos();
            let new_format = match &self.line_datas[cur_pos.row].result_format {
                ResultFormat::Bin => ResultFormat::Hex,
                ResultFormat::Dec => ResultFormat::Bin,
                ResultFormat::Hex => ResultFormat::Dec,
            };
            self.line_datas[cur_pos.row].result_format = new_format;
        } else if modifiers.alt && input == InputKey::Right {
            let cur_pos = self.editor.get_selection().get_cursor_pos();
            let new_format = match &self.line_datas[cur_pos.row].result_format {
                ResultFormat::Bin => ResultFormat::Dec,
                ResultFormat::Dec => ResultFormat::Hex,
                ResultFormat::Hex => ResultFormat::Bin,
            };
            self.line_datas[cur_pos.row].result_format = new_format;
        } else {
            self.editor
                .handle_input(input, modifiers, &mut self.line_datas);
        }
    }
}

fn digit_count(n: usize) -> usize {
    let mut n = n;
    let mut count = 1;
    while n > 9 {
        count += 1;
        n = n / 10;
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asd() {
        let mut app = NoteCalcApp::new(120);
        app.handle_input(InputKey::Text("\n[3;4;2]\n"), InputModifiers::none());
        app.render();
    }
}
