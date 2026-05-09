use crate::types::RuleConfig;
use std::collections::BTreeMap;

const THEMATIC_BREAK_MIN_MARKERS: usize = 3;

#[derive(Debug, Clone, Copy)]
pub(super) struct OrderedListState {
    pub(super) next_number: u32,
    pub(super) content_indent: usize,
    detected_style: Option<DetectedOrderedListStyle>,
}

impl Default for OrderedListState {
    fn default() -> Self {
        Self {
            next_number: 1,
            content_indent: 0,
            detected_style: None,
        }
    }
}

impl OrderedListState {
    pub(super) fn expected_number(&mut self, actual: u32, style: OrderedListStyle) -> Option<u32> {
        match style {
            OrderedListStyle::One => self.expected_constant_number(actual, 1),
            OrderedListStyle::Ordered => self.expected_ordered_number(actual),
            OrderedListStyle::OneOrOrdered => self.expected_one_or_ordered(actual),
            OrderedListStyle::Zero => self.expected_constant_number(actual, 0),
        }
    }

    fn expected_constant_number(&mut self, actual: u32, expected: u32) -> Option<u32> {
        self.next_number += 1;
        mismatch_expected(actual, expected)
    }

    fn expected_ordered_number(&mut self, actual: u32) -> Option<u32> {
        let expected = self.next_number;
        self.next_number += 1;
        mismatch_expected(actual, expected)
    }

    fn expected_one_or_ordered(&mut self, actual: u32) -> Option<u32> {
        if self.next_number == 1 && self.detected_style.is_none() {
            return self.detect_initial_one_or_ordered(actual);
        }
        self.continue_one_or_ordered(actual)
    }

    fn detect_initial_one_or_ordered(&mut self, actual: u32) -> Option<u32> {
        if actual == 0 {
            self.detected_style = Some(DetectedOrderedListStyle::Ordered);
            return None;
        }
        self.next_number = 2;
        mismatch_expected(actual, 1)
    }

    fn continue_one_or_ordered(&mut self, actual: u32) -> Option<u32> {
        match self.detected_style {
            Some(DetectedOrderedListStyle::One) => self.expected_constant_number(actual, 1),
            Some(DetectedOrderedListStyle::Ordered) => self.expected_ordered_number(actual),
            None if actual == 1 => {
                self.detected_style = Some(DetectedOrderedListStyle::One);
                self.next_number += 1;
                None
            }
            None if actual == self.next_number => {
                self.detected_style = Some(DetectedOrderedListStyle::Ordered);
                self.next_number += 1;
                None
            }
            None => {
                let expected = self.next_number;
                self.detected_style = Some(DetectedOrderedListStyle::Ordered);
                self.next_number += 1;
                Some(expected)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) enum OrderedListStyle {
    One,
    Ordered,
    OneOrOrdered,
    Zero,
}

impl OrderedListStyle {
    pub(super) fn from_config(config: Option<&RuleConfig>) -> Self {
        match config
            .and_then(|config| config.properties.get("style"))
            .map(String::as_str)
        {
            Some("one") => Self::One,
            Some("ordered") => Self::Ordered,
            Some("zero") => Self::Zero,
            _ => Self::OneOrOrdered,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum DetectedOrderedListStyle {
    One,
    Ordered,
}

fn mismatch_expected(actual: u32, expected: u32) -> Option<u32> {
    (actual != expected).then_some(expected)
}

pub(super) fn ordered_marker_width(trimmed: &str) -> usize {
    trimmed.find(". ").map_or(0, |dot_pos| dot_pos + 2)
}

pub(super) fn is_ordered_list_continuation(
    indent: usize,
    expected_numbers: &BTreeMap<usize, OrderedListState>,
) -> bool {
    expected_numbers
        .values()
        .any(|state| state.content_indent > 0 && indent >= state.content_indent)
}

pub(super) fn is_ordered_list_lazy_continuation(
    indent: usize,
    trimmed: &str,
    expected_numbers: &BTreeMap<usize, OrderedListState>,
) -> bool {
    if is_ordered_list_boundary(trimmed) {
        return false;
    }
    expected_numbers
        .iter()
        .any(|(level, state)| state.content_indent > 0 && indent >= *level)
}

fn is_ordered_list_boundary(trimmed: &str) -> bool {
    trimmed.starts_with('#')
        || trimmed.starts_with('>')
        || trimmed.starts_with("```")
        || trimmed.starts_with("~~~")
        || is_thematic_break(trimmed)
}

fn is_thematic_break(trimmed: &str) -> bool {
    let mut marker: Option<char> = None;
    let mut count = 0;
    for ch in trimmed.chars() {
        if ch == ' ' || ch == '\t' {
            continue;
        }
        if ch != '-' && ch != '_' && ch != '*' {
            return false;
        }
        match marker {
            Some(existing) if existing != ch => return false,
            None => marker = Some(ch),
            _ => {}
        }
        count += 1;
    }
    count >= THEMATIC_BREAK_MIN_MARKERS
}
