use crate::palette::{BaseScale, Palette};
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_workspace_rule("editor.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rules(
        &["editor.foreground", "foreground"],
        palette.base(BaseScale::Fg),
    );

    builder.add_workspace_rule(
        "editorCodeLens.foreground",
        palette.base(BaseScale::FadedFg),
    );

    builder.add_workspace_rule(
        "rust_analyzer.inlayHints.foreground",
        palette.base(BaseScale::FadedFg),
    );
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rule(Semantic("keyword"), palette.purple());

    builder.add_rules(&[Semantic("function"), Semantic("method")], palette.blue());

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("enum"),
            Semantic("union"),
            Semantic("interface"),
            Semantic("builtinType"),
            Semantic("typeAlias"),
            Semantic("typeParameter"),
        ],
        palette.yellow(),
    );

    builder.add_rule(Semantic("enumMember"), palette.green());

    builder.add_rule(Semantic("property"), palette.red());

    builder.add_rules(
        &[Semantic("string"), Semantic("character")],
        palette.green(),
    );

    builder.add_rule(Semantic("number"), palette.orange());
    builder.add_rules(
        &[Semantic("variable.constant"), Semantic("variable.static")],
        palette.orange(),
    );

    builder.add_rule(Semantic("escapeSequence"), palette.cyan());
    builder.add_rule(Semantic("formatSpecifier"), palette.brown());

    builder.add_rule(Semantic("macro"), palette.cyan());

    builder.add_rule(Semantic("lifetime"), palette.brown());

    builder.add_rule(
        Semantic("comment"),
        (palette.base(BaseScale::BrightFg), FontStyle::Italic),
    );
}
