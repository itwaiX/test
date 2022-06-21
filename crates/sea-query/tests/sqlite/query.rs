use super::*;
use pretty_assertions::assert_eq;

#[test]
fn select_1() {
    assert_eq!(
        Query::select()
            .columns([Char::Character, Char::SizeW, Char::SizeH])
            .from(Char::Table)
            .limit(10)
            .offset(100)
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character", "size_w", "size_h" FROM "character" LIMIT 10 OFFSET 100"#
    );
}

#[test]
fn select_2() {
    assert_eq!(
        Query::select()
            .columns([Char::Character, Char::SizeW, Char::SizeH])
            .from(Char::Table)
            .and_where(Expr::col(Char::SizeW).eq(3))
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 3"#
    );
}

#[test]
fn select_3() {
    assert_eq!(
        Query::select()
            .columns([Char::Character, Char::SizeW, Char::SizeH])
            .from(Char::Table)
            .and_where(Expr::col(Char::SizeW).eq(3))
            .and_where(Expr::col(Char::SizeH).eq(4))
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 3 AND "size_h" = 4"#
    );
}

#[test]
fn select_4() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Image])
            .from_subquery(
                Query::select()
                    .columns([Glyph::Image, Glyph::Aspect])
                    .from(Glyph::Table)
                    .take(),
                Alias::new("subglyph")
            )
            .to_string(SqliteQueryBuilder),
        r#"SELECT "image" FROM (SELECT "image", "aspect" FROM "glyph") AS "subglyph""#
    );
}

#[test]
fn select_5() {
    assert_eq!(
        Query::select()
            .column((Glyph::Table, Glyph::Image))
            .from(Glyph::Table)
            .and_where(Expr::tbl(Glyph::Table, Glyph::Aspect).is_in(vec![3, 4]))
            .to_string(SqliteQueryBuilder),
        r#"SELECT "glyph"."image" FROM "glyph" WHERE "glyph"."aspect" IN (3, 4)"#
    );
}

#[test]
fn select_6() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect,])
            .exprs(vec![Expr::col(Glyph::Image).max(),])
            .from(Glyph::Table)
            .group_by_columns(vec![Glyph::Aspect,])
            .and_having(Expr::col(Glyph::Aspect).gt(2))
            .to_string(SqliteQueryBuilder),
        r#"SELECT "aspect", MAX("image") FROM "glyph" GROUP BY "aspect" HAVING "aspect" > 2"#
    );
}

#[test]
fn select_7() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect,])
            .from(Glyph::Table)
            .and_where(Expr::expr(Expr::col(Glyph::Aspect).if_null(0)).gt(2))
            .to_string(SqliteQueryBuilder),
        r#"SELECT "aspect" FROM "glyph" WHERE IFNULL("aspect", 0) > 2"#
    );
}

#[test]
fn select_8() {
    assert_eq!(
        Query::select()
            .columns([Char::Character])
            .from(Char::Table)
            .left_join(
                Font::Table,
                Expr::tbl(Char::Table, Char::FontId).equals(Font::Table, Font::Id),
            )
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id""#
    );
}

#[test]
fn select_9() {
    assert_eq!(
        Query::select()
            .columns([Char::Character])
            .from(Char::Table)
            .left_join(
                Font::Table,
                Expr::tbl(Char::Table, Char::FontId).equals(Font::Table, Font::Id),
            )
            .inner_join(
                Glyph::Table,
                Expr::tbl(Char::Table, Char::Character).equals(Glyph::Table, Glyph::Image),
            )
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id" INNER JOIN "glyph" ON "character"."character" = "glyph"."image""#
    );
}

#[test]
fn select_10() {
    assert_eq!(
        Query::select()
            .columns([Char::Character])
            .from(Char::Table)
            .left_join(
                Font::Table,
                Expr::tbl(Char::Table, Char::FontId)
                    .equals(Font::Table, Font::Id)
                    .and(Expr::tbl(Char::Table, Char::FontId).equals(Font::Table, Font::Id)),
            )
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character" FROM "character" LEFT JOIN "font" ON ("character"."font_id" = "font"."id") AND ("character"."font_id" = "font"."id")"#
    );
}

#[test]
fn select_11() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect])
            .from(Glyph::Table)
            .and_where(Expr::expr(Expr::col(Glyph::Aspect).if_null(0)).gt(2))
            .order_by(Glyph::Image, Order::Desc)
            .order_by((Glyph::Table, Glyph::Aspect), Order::Asc)
            .to_string(SqliteQueryBuilder),
        r#"SELECT "aspect" FROM "glyph" WHERE IFNULL("aspect", 0) > 2 ORDER BY "image" DESC, "glyph"."aspect" ASC"#
    );
}

#[test]
fn select_12() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect])
            .from(Glyph::Table)
            .and_where(Expr::expr(Expr::col(Glyph::Aspect).if_null(0)).gt(2))
            .order_by_columns(vec![(Glyph::Id, Order::Asc), (Glyph::Aspect, Order::Desc)])
            .to_string(SqliteQueryBuilder),
        r#"SELECT "aspect" FROM "glyph" WHERE IFNULL("aspect", 0) > 2 ORDER BY "id" ASC, "aspect" DESC"#
    );
}

#[test]
fn select_13() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect])
            .from(Glyph::Table)
            .and_where(Expr::expr(Expr::col(Glyph::Aspect).if_null(0)).gt(2))
            .order_by_columns(vec![
                ((Glyph::Table, Glyph::Id), Order::Asc),
                ((Glyph::Table, Glyph::Aspect), Order::Desc),
            ])
            .to_string(SqliteQueryBuilder),
        r#"SELECT "aspect" FROM "glyph" WHERE IFNULL("aspect", 0) > 2 ORDER BY "glyph"."id" ASC, "glyph"."aspect" DESC"#
    );
}

#[test]
fn select_14() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Id, Glyph::Aspect])
            .expr(Expr::col(Glyph::Image).max())
            .from(Glyph::Table)
            .group_by_columns(vec![
                (Glyph::Table, Glyph::Id),
                (Glyph::Table, Glyph::Aspect),
            ])
            .and_having(Expr::col(Glyph::Aspect).gt(2))
            .to_string(SqliteQueryBuilder),
        r#"SELECT "id", "aspect", MAX("image") FROM "glyph" GROUP BY "glyph"."id", "glyph"."aspect" HAVING "aspect" > 2"#
    );
}

#[test]
fn select_15() {
    assert_eq!(
        Query::select()
            .columns([Char::Character])
            .from(Char::Table)
            .and_where(Expr::col(Char::FontId).is_null())
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE "font_id" IS NULL"#
    );
}

#[test]
fn select_16() {
    assert_eq!(
        Query::select()
            .columns([Char::Character])
            .from(Char::Table)
            .and_where(Expr::col(Char::FontId).is_null())
            .and_where(Expr::col(Char::Character).is_not_null())
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE "font_id" IS NULL AND "character" IS NOT NULL"#
    );
}

#[test]
fn select_17() {
    assert_eq!(
        Query::select()
            .columns([(Glyph::Table, Glyph::Image),])
            .from(Glyph::Table)
            .and_where(Expr::tbl(Glyph::Table, Glyph::Aspect).between(3, 5))
            .to_string(SqliteQueryBuilder),
        r#"SELECT "glyph"."image" FROM "glyph" WHERE "glyph"."aspect" BETWEEN 3 AND 5"#
    );
}

#[test]
fn select_18() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect])
            .from(Glyph::Table)
            .and_where(Expr::col(Glyph::Aspect).between(3, 5))
            .and_where(Expr::col(Glyph::Aspect).not_between(8, 10))
            .to_string(SqliteQueryBuilder),
        r#"SELECT "aspect" FROM "glyph" WHERE ("aspect" BETWEEN 3 AND 5) AND ("aspect" NOT BETWEEN 8 AND 10)"#
    );
}

#[test]
fn select_19() {
    assert_eq!(
        Query::select()
            .columns([Char::Character])
            .from(Char::Table)
            .and_where(Expr::col(Char::Character).eq("A"))
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE "character" = 'A'"#
    );
}

#[test]
fn select_20() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .and_where(Expr::col(Char::Character).like("A"))
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE "character" LIKE 'A'"#
    );
}

#[test]
fn select_21() {
    assert_eq!(
        Query::select()
            .columns([Char::Character])
            .from(Char::Table)
            .or_where(Expr::col(Char::Character).like("A%"))
            .or_where(Expr::col(Char::Character).like("%B"))
            .or_where(Expr::col(Char::Character).like("%C%"))
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE "character" LIKE 'A%' OR "character" LIKE '%B' OR "character" LIKE '%C%'"#
    );
}

#[test]
fn select_22() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .cond_where(
                Cond::all()
                    .add(
                        Cond::any().add(Expr::col(Char::Character).like("C")).add(
                            Expr::col(Char::Character)
                                .like("D")
                                .and(Expr::col(Char::Character).like("E"))
                        )
                    )
                    .add(
                        Expr::col(Char::Character)
                            .like("F")
                            .or(Expr::col(Char::Character).like("G"))
                    )
            )
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE ("character" LIKE 'C' OR (("character" LIKE 'D') AND ("character" LIKE 'E'))) AND (("character" LIKE 'F') OR ("character" LIKE 'G'))"#
    );
}

#[test]
fn select_23() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .and_where_option(None)
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character" FROM "character""#
    );
}

#[test]
fn select_24() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .conditions(
                true,
                |x| {
                    x.and_where(Expr::col(Char::FontId).eq(5));
                },
                |_| ()
            )
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE "font_id" = 5"#
    );
}

#[test]
fn select_25() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .and_where(
                Expr::col(Char::SizeW)
                    .mul(2)
                    .equals(Expr::col(Char::SizeH).div(2))
            )
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE "size_w" * 2 = "size_h" / 2"#
    );
}

#[test]
fn select_26() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .and_where(
                Expr::expr(Expr::col(Char::SizeW).add(1))
                    .mul(2)
                    .equals(Expr::expr(Expr::col(Char::SizeH).div(2)).sub(1))
            )
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE ("size_w" + 1) * 2 = ("size_h" / 2) - 1"#
    );
}

#[test]
fn select_27() {
    assert_eq!(
        Query::select()
            .columns([Char::Character, Char::SizeW, Char::SizeH])
            .from(Char::Table)
            .and_where(Expr::col(Char::SizeW).eq(3))
            .and_where(Expr::col(Char::SizeH).eq(4))
            .and_where(Expr::col(Char::SizeH).eq(5))
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 3 AND "size_h" = 4 AND "size_h" = 5"#
    );
}

#[test]
fn select_28() {
    assert_eq!(
        Query::select()
            .columns([Char::Character, Char::SizeW, Char::SizeH])
            .from(Char::Table)
            .or_where(Expr::col(Char::SizeW).eq(3))
            .or_where(Expr::col(Char::SizeH).eq(4))
            .or_where(Expr::col(Char::SizeH).eq(5))
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 3 OR "size_h" = 4 OR "size_h" = 5"#
    );
}

#[test]
#[should_panic]
fn select_29() {
    assert_eq!(
        Query::select()
            .columns([Char::Character, Char::SizeW, Char::SizeH])
            .from(Char::Table)
            .and_where(Expr::col(Char::SizeW).eq(3))
            .or_where(Expr::col(Char::SizeH).eq(4))
            .and_where(Expr::col(Char::SizeH).eq(5))
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 3 OR "size_h" = 4 AND "size_h" = 5"#
    );
}

#[test]
fn select_30() {
    assert_eq!(
        Query::select()
            .columns([Char::Character, Char::SizeW, Char::SizeH])
            .from(Char::Table)
            .and_where(
                Expr::col(Char::SizeW)
                    .mul(2)
                    .add(Expr::col(Char::SizeH).div(3))
                    .equals(Expr::value(4))
            )
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w" * 2) + ("size_h" / 3) = 4"#
    );
}

#[test]
fn select_31() {
    assert_eq!(
        Query::select()
            .expr((1..10_i32).fold(Expr::value(0), |expr, i| { expr.add(Expr::value(i)) }))
            .to_string(SqliteQueryBuilder),
        r#"SELECT 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9"#
    );
}

#[test]
fn select_32() {
    assert_eq!(
        Query::select()
            .expr_as(Expr::col(Char::Character), Alias::new("C"))
            .from(Char::Table)
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character" AS "C" FROM "character""#
    );
}

#[test]
fn select_33() {
    assert_eq!(
        Query::select()
            .column(Glyph::Image)
            .from(Glyph::Table)
            .and_where(
                Expr::col(Glyph::Aspect)
                    .in_subquery(Query::select().expr(Expr::cust("3 + 2 * 2")).take())
            )
            .to_string(SqliteQueryBuilder),
        r#"SELECT "image" FROM "glyph" WHERE "aspect" IN (SELECT 3 + 2 * 2)"#
    );
}

#[test]
fn select_34a() {
    assert_eq!(
        Query::select()
            .column(Glyph::Aspect)
            .expr(Expr::col(Glyph::Image).max())
            .from(Glyph::Table)
            .group_by_columns(vec![Glyph::Aspect,])
            .or_having(
                Expr::col(Glyph::Aspect)
                    .gt(2)
                    .or(Expr::col(Glyph::Aspect).lt(8))
            )
            .or_having(
                Expr::col(Glyph::Aspect)
                    .gt(12)
                    .and(Expr::col(Glyph::Aspect).lt(18))
            )
            .or_having(Expr::col(Glyph::Aspect).gt(32))
            .to_string(SqliteQueryBuilder),
        vec![
            r#"SELECT "aspect", MAX("image") FROM "glyph" GROUP BY "aspect""#,
            r#"HAVING (("aspect" > 2) OR ("aspect" < 8))"#,
            r#"OR (("aspect" > 12) AND ("aspect" < 18))"#,
            r#"OR "aspect" > 32"#,
        ]
        .join(" ")
    );
}

#[test]
#[should_panic]
fn select_34b() {
    assert_eq!(
        Query::select()
            .column(Glyph::Aspect)
            .expr(Expr::col(Glyph::Image).max())
            .from(Glyph::Table)
            .group_by_columns(vec![Glyph::Aspect,])
            .or_having(
                Expr::col(Glyph::Aspect)
                    .gt(2)
                    .or(Expr::col(Glyph::Aspect).lt(8))
            )
            .and_having(
                Expr::col(Glyph::Aspect)
                    .gt(22)
                    .or(Expr::col(Glyph::Aspect).lt(28))
            )
            .to_string(SqliteQueryBuilder),
        vec![
            r#"SELECT "aspect", MAX("image") FROM "glyph" GROUP BY "aspect""#,
            r#"HAVING (("aspect" > 2) OR ("aspect" < 8))"#,
            r#"AND (("aspect" > 22) OR ("aspect" < 28))"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_35() {
    let (statement, values) = sea_query::Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .and_where(Expr::col(Glyph::Aspect).is_null())
        .build(sea_query::SqliteQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE "aspect" IS NULL"#
    );
    assert_eq!(values.0, vec![]);
}

#[test]
fn select_36() {
    let (statement, values) = sea_query::Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(Cond::any().add(Expr::col(Glyph::Aspect).is_null()))
        .build(sea_query::SqliteQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE "aspect" IS NULL"#
    );
    assert_eq!(values.0, vec![]);
}

#[test]
fn select_37() {
    let (statement, values) = sea_query::Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(Cond::any().add(Cond::all()).add(Cond::any()))
        .build(sea_query::SqliteQueryBuilder);

    assert_eq!(statement, r#"SELECT "id" FROM "glyph""#);
    assert_eq!(values.0, vec![]);
}

#[test]
fn select_38() {
    let (statement, values) = sea_query::Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::any()
                .add(Expr::col(Glyph::Aspect).is_null())
                .add(Expr::col(Glyph::Aspect).is_not_null()),
        )
        .build(sea_query::SqliteQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE "aspect" IS NULL OR "aspect" IS NOT NULL"#
    );
    assert_eq!(values.0, vec![]);
}

#[test]
fn select_39() {
    let (statement, values) = sea_query::Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::all()
                .add(Expr::col(Glyph::Aspect).is_null())
                .add(Expr::col(Glyph::Aspect).is_not_null()),
        )
        .build(sea_query::SqliteQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE "aspect" IS NULL AND "aspect" IS NOT NULL"#
    );
    assert_eq!(values.0, vec![]);
}

#[test]
fn select_40() {
    let statement = sea_query::Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(any![
            Expr::col(Glyph::Aspect).is_null(),
            all![
                Expr::col(Glyph::Aspect).is_not_null(),
                Expr::col(Glyph::Aspect).lt(8)
            ]
        ])
        .to_string(sea_query::SqliteQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE "aspect" IS NULL OR ("aspect" IS NOT NULL AND "aspect" < 8)"#
    );
}

#[test]
fn select_41() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect])
            .exprs(vec![Expr::col(Glyph::Image).max()])
            .from(Glyph::Table)
            .group_by_columns(vec![Glyph::Aspect])
            .cond_having(any![Expr::col(Glyph::Aspect).gt(2)])
            .to_string(SqliteQueryBuilder),
        r#"SELECT "aspect", MAX("image") FROM "glyph" GROUP BY "aspect" HAVING "aspect" > 2"#
    );
}

#[test]
fn select_42() {
    let statement = sea_query::Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::all()
                .add_option(Some(Expr::col(Glyph::Aspect).lt(8)))
                .add(Expr::col(Glyph::Aspect).is_not_null()),
        )
        .to_string(SqliteQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE "aspect" < 8 AND "aspect" IS NOT NULL"#
    );
}

#[test]
fn select_43() {
    let statement = sea_query::Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(Cond::all().add_option::<SimpleExpr>(None))
        .to_string(SqliteQueryBuilder);

    assert_eq!(statement, r#"SELECT "id" FROM "glyph""#);
}

#[test]
fn select_44() {
    let statement = sea_query::Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::any()
                .not()
                .add_option(Some(Expr::col(Glyph::Aspect).lt(8))),
        )
        .to_string(SqliteQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE NOT ("aspect" < 8)"#
    );
}

#[test]
fn select_45() {
    let statement = sea_query::Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::any()
                .not()
                .add_option(Some(Expr::col(Glyph::Aspect).lt(8)))
                .add(Expr::col(Glyph::Aspect).is_not_null()),
        )
        .to_string(SqliteQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE NOT ("aspect" < 8 OR "aspect" IS NOT NULL)"#
    );
}

#[test]
fn select_46() {
    let statement = sea_query::Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::all()
                .not()
                .add_option(Some(Expr::col(Glyph::Aspect).lt(8))),
        )
        .to_string(SqliteQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE NOT ("aspect" < 8)"#
    );
}

#[test]
fn select_47() {
    let statement = sea_query::Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::all()
                .not()
                .add_option(Some(Expr::col(Glyph::Aspect).lt(8)))
                .add(Expr::col(Glyph::Aspect).is_not_null()),
        )
        .to_string(SqliteQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE NOT ("aspect" < 8 AND "aspect" IS NOT NULL)"#
    );
}

#[test]
fn select_48() {
    let statement = sea_query::Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::all().add_option(
                Some(
                    ConditionExpression::SimpleExpr(
                        Expr::tuple([
                            Expr::col(Glyph::Aspect).into_simple_expr(),
                            Expr::value(100),
                        ])
                        .less_than(Expr::tuple([Expr::value(8), Expr::value(100)])),
                    ),
                ),
            ),
        )
        .to_string(SqliteQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE ("aspect", 100) < (8, 100)"#
    );
}

#[test]
fn select_49() {
    let statement = sea_query::Query::select()
        .expr(Expr::asterisk())
        .from(Char::Table)
        .to_string(SqliteQueryBuilder);

    assert_eq!(statement, r#"SELECT * FROM "character""#);
}

#[test]
fn select_50() {
    let statement = sea_query::Query::select()
        .expr(Expr::table_asterisk(Char::Table))
        .column((Font::Table, Font::Name))
        .from(Char::Table)
        .inner_join(
            Font::Table,
            Expr::tbl(Char::Table, Char::FontId).equals(Font::Table, Font::Id),
        )
        .to_string(SqliteQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "character".*, "font"."name" FROM "character" INNER JOIN "font" ON "character"."font_id" = "font"."id""#
    )
}

#[test]
fn select_51() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect,])
            .from(Glyph::Table)
            .and_where(Expr::expr(Expr::col(Glyph::Aspect).if_null(0)).gt(2))
            .order_by_with_nulls(Glyph::Image, Order::Desc, NullOrdering::First)
            .order_by_with_nulls(
                (Glyph::Table, Glyph::Aspect),
                Order::Asc,
                NullOrdering::Last
            )
            .to_string(SqliteQueryBuilder),
        [
            r#"SELECT "aspect""#,
            r#"FROM "glyph""#,
            r#"WHERE IFNULL("aspect", 0) > 2"#,
            r#"ORDER BY "image" DESC NULLS FIRST,"#,
            r#""glyph"."aspect" ASC NULLS LAST"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_52() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect,])
            .from(Glyph::Table)
            .and_where(Expr::expr(Expr::col(Glyph::Aspect).if_null(0)).gt(2))
            .order_by_columns_with_nulls(vec![
                (Glyph::Id, Order::Asc, NullOrdering::First),
                (Glyph::Aspect, Order::Desc, NullOrdering::Last),
            ])
            .to_string(SqliteQueryBuilder),
        [
            r#"SELECT "aspect""#,
            r#"FROM "glyph""#,
            r#"WHERE IFNULL("aspect", 0) > 2"#,
            r#"ORDER BY "id" ASC NULLS FIRST,"#,
            r#""aspect" DESC NULLS LAST"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_53() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect,])
            .from(Glyph::Table)
            .and_where(Expr::expr(Expr::col(Glyph::Aspect).if_null(0)).gt(2))
            .order_by_columns_with_nulls(vec![
                ((Glyph::Table, Glyph::Id), Order::Asc, NullOrdering::First),
                (
                    (Glyph::Table, Glyph::Aspect),
                    Order::Desc,
                    NullOrdering::Last
                ),
            ])
            .to_string(SqliteQueryBuilder),
        [
            r#"SELECT "aspect""#,
            r#"FROM "glyph""#,
            r#"WHERE IFNULL("aspect", 0) > 2"#,
            r#"ORDER BY "glyph"."id" ASC NULLS FIRST,"#,
            r#""glyph"."aspect" DESC NULLS LAST"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_54() {
    let statement = sea_query::Query::select()
        .expr(Expr::asterisk())
        .from(Char::Table)
        .from(Font::Table)
        .and_where(Expr::tbl(Font::Table, Font::Id).equals(Char::Table, Char::FontId))
        .to_string(SqliteQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT * FROM "character", "font" WHERE "font"."id" = "character"."font_id""#
    );
}

#[test]
fn select_55() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect,])
            .from(Glyph::Table)
            .and_where(Expr::expr(Expr::col(Glyph::Aspect).if_null(0)).gt(2))
            .order_by(
                Glyph::Id,
                Order::Field(Values(vec![
                    Value::Int(Some(4)),
                    Value::Int(Some(5)),
                    Value::Int(Some(1)),
                    Value::Int(Some(3))
                ]))
            )
            .order_by((Glyph::Table, Glyph::Aspect), Order::Asc)
            .to_string(SqliteQueryBuilder),
        [
            r#"SELECT "aspect""#,
            r#"FROM "glyph""#,
            r#"WHERE IFNULL("aspect", 0) > 2"#,
            r#"ORDER BY CASE"#,
            r#"WHEN "id"=4 THEN 0"#,
            r#"WHEN "id"=5 THEN 1"#,
            r#"WHEN "id"=1 THEN 2"#,
            r#"WHEN "id"=3 THEN 3"#,
            r#"ELSE 4 END,"#,
            r#""glyph"."aspect" ASC"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_56() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect,])
            .from(Glyph::Table)
            .and_where(Expr::expr(Expr::col(Glyph::Aspect).if_null(0)).gt(2))
            .order_by((Glyph::Table, Glyph::Aspect), Order::Asc)
            .order_by(
                Glyph::Id,
                Order::Field(Values(vec![
                    Value::Int(Some(4)),
                    Value::Int(Some(5)),
                    Value::Int(Some(1)),
                    Value::Int(Some(3))
                ]))
            )
            .to_string(SqliteQueryBuilder),
        [
            r#"SELECT "aspect""#,
            r#"FROM "glyph""#,
            r#"WHERE IFNULL("aspect", 0) > 2"#,
            r#"ORDER BY "glyph"."aspect" ASC,"#,
            r#"CASE WHEN "id"=4 THEN 0"#,
            r#"WHEN "id"=5 THEN 1"#,
            r#"WHEN "id"=1 THEN 2"#,
            r#"WHEN "id"=3 THEN 3"#,
            r#"ELSE 4 END"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_57() {
    let query = Query::select()
        .expr_as(
            CaseStatement::new()
                .case(
                    Expr::tbl(Glyph::Table, Glyph::Aspect).gt(0),
                    Expr::val("positive"),
                )
                .case(
                    Expr::tbl(Glyph::Table, Glyph::Aspect).lt(0),
                    Expr::val("negative"),
                )
                .finally(Expr::val("zero")),
            Alias::new("polarity"),
        )
        .from(Glyph::Table)
        .to_owned();

    assert_eq!(
        query.to_string(SqliteQueryBuilder),
        r#"SELECT (CASE WHEN ("glyph"."aspect" > 0) THEN 'positive' WHEN ("glyph"."aspect" < 0) THEN 'negative' ELSE 'zero' END) AS "polarity" FROM "glyph""#
    );
}

#[test]
fn select_58() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .and_where(Expr::col(Char::Character).like(LikeExpr::str("A").escape('\\')))
            .to_string(SqliteQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE "character" LIKE 'A' ESCAPE '\'"#
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_2() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Image, Glyph::Aspect])
            .values_panic(vec![
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .to_string(SqliteQueryBuilder),
        r#"INSERT INTO "glyph" ("image", "aspect") VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_3() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Image, Glyph::Aspect])
            .values_panic(vec![
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .values_panic(vec![Value::Double(None), 2.1345.into()])
            .to_string(SqliteQueryBuilder),
        r#"INSERT INTO "glyph" ("image", "aspect") VALUES ('04108048005887010020060000204E0180400400', 3.1415), (NULL, 2.1345)"#
    );
}

#[test]
#[cfg(feature = "with-chrono")]
fn insert_4() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Image])
            .values_panic(vec![chrono::NaiveDateTime::from_timestamp(0, 0).into()])
            .to_string(SqliteQueryBuilder),
        r#"INSERT INTO "glyph" ("image") VALUES ('1970-01-01 00:00:00')"#
    );
}

#[test]
#[cfg(feature = "with-time")]
fn insert_8() {
    use time::{date, time};
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Image])
            .values_panic(vec![date!(1970 - 01 - 01)
                .with_time(time!(00:00:00))
                .into()])
            .to_string(SqliteQueryBuilder),
        r#"INSERT INTO "glyph" ("image") VALUES ('1970-01-01 00:00:00')"#
    );
}

#[test]
fn insert_from_select() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Aspect, Glyph::Image])
            .select_from(
                Query::select()
                    .column(Glyph::Aspect)
                    .column(Glyph::Image)
                    .from(Glyph::Table)
                    .conditions(
                        true,
                        |x| {
                            x.and_where(Expr::col(Glyph::Image).like("%"));
                        },
                        |x| {
                            x.and_where(Expr::col(Glyph::Id).eq(6));
                        },
                    )
                    .to_owned()
            )
            .unwrap()
            .to_owned()
            .to_string(SqliteQueryBuilder),
        r#"INSERT INTO "glyph" ("aspect", "image") SELECT "aspect", "image" FROM "glyph" WHERE "image" LIKE '%'"#
    );
}

#[test]
#[cfg(feature = "with-uuid")]
fn insert_5() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Image])
            .values_panic(vec![uuid::Uuid::nil().into()])
            .to_string(SqliteQueryBuilder),
        r#"INSERT INTO "glyph" ("image") VALUES ('00000000-0000-0000-0000-000000000000')"#
    );
}

#[test]
fn insert_6() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .or_default_values()
            .to_string(SqliteQueryBuilder),
        r#"INSERT INTO "glyph" DEFAULT VALUES"#
    );
}

#[test]
fn insert_7() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .or_default_values()
            .returning_col(Glyph::Id)
            .to_string(SqliteQueryBuilder),
        r#"INSERT INTO "glyph" DEFAULT VALUES RETURNING "id""#
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_1() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Aspect, Glyph::Image])
            .values_panic(vec![
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::column(Glyph::Id)
                    .update_column(Glyph::Aspect)
                    .to_owned()
            )
            .to_string(SqliteQueryBuilder),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id") DO UPDATE SET "aspect" = "excluded"."aspect""#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_2() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Aspect, Glyph::Image])
            .values_panic(vec![
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::columns([Glyph::Id, Glyph::Aspect])
                    .update_columns([Glyph::Aspect, Glyph::Image])
                    .to_owned()
            )
            .to_string(SqliteQueryBuilder),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO UPDATE SET "aspect" = "excluded"."aspect", "image" = "excluded"."image""#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_3() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Aspect, Glyph::Image])
            .values_panic(vec![
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::columns([Glyph::Id, Glyph::Aspect])
                    .update_values([
                        (Glyph::Aspect, "04108048005887010020060000204E0180400400".into()),
                        (Glyph::Image, 3.1415.into()),
                    ])
                    .to_owned()
            )
            .to_string(SqliteQueryBuilder),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO UPDATE SET "aspect" = '04108048005887010020060000204E0180400400', "image" = 3.1415"#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_4() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Aspect, Glyph::Image])
            .values_panic(vec![
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::columns([Glyph::Id, Glyph::Aspect])
                    .update_expr((Glyph::Image, Expr::val(1).add(2)))
                    .to_owned()
            )
            .to_string(SqliteQueryBuilder),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO UPDATE SET "image" = 1 + 2"#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_returning_all_columns() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Image, Glyph::Aspect,])
            .values_panic(vec![
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .returning(Query::returning().all())
            .to_string(SqliteQueryBuilder),
        r#"INSERT INTO "glyph" ("image", "aspect") VALUES ('04108048005887010020060000204E0180400400', 3.1415) RETURNING *"#
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_returning_specific_columns() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Image, Glyph::Aspect,])
            .values_panic(vec![
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .returning(Query::returning().columns([Glyph::Id, Glyph::Image]))
            .to_string(SqliteQueryBuilder),
        r#"INSERT INTO "glyph" ("image", "aspect") VALUES ('04108048005887010020060000204E0180400400', 3.1415) RETURNING "id", "image""#
    );
}

#[test]
fn update_1() {
    assert_eq!(
        Query::update()
            .table(Glyph::Table)
            .values(vec![
                (Glyph::Aspect, 2.1345.into()),
                (
                    Glyph::Image,
                    "24B0E11951B03B07F8300FD003983F03F0780060".into()
                ),
            ])
            .and_where(Expr::col(Glyph::Id).eq(1))
            .to_string(SqliteQueryBuilder),
        r#"UPDATE "glyph" SET "aspect" = 2.1345, "image" = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE "id" = 1"#
    );
}

#[test]
fn update_3() {
    assert_eq!(
        Query::update()
            .table(Glyph::Table)
            .value_expr(Glyph::Aspect, Expr::cust("60 * 24 * 24"))
            .values(vec![(
                Glyph::Image,
                "24B0E11951B03B07F8300FD003983F03F0780060".into()
            ),])
            .and_where(Expr::col(Glyph::Id).eq(1))
            .to_string(SqliteQueryBuilder),
        r#"UPDATE "glyph" SET "aspect" = 60 * 24 * 24, "image" = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE "id" = 1"#
    );
}

#[test]
fn update_returning_all_columns() {
    assert_eq!(
        Query::update()
            .table(Glyph::Table)
            .value_expr(Glyph::Aspect, Expr::cust("60 * 24 * 24"))
            .values(vec![(
                Glyph::Image,
                "24B0E11951B03B07F8300FD003983F03F0780060".into()
            ),])
            .and_where(Expr::col(Glyph::Id).eq(1))
            .returning(Query::returning().all())
            .to_string(SqliteQueryBuilder),
        r#"UPDATE "glyph" SET "aspect" = 60 * 24 * 24, "image" = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE "id" = 1 RETURNING *"#
    );
}

#[test]
fn update_returning_specified_columns() {
    assert_eq!(
        Query::update()
            .table(Glyph::Table)
            .value_expr(Glyph::Aspect, Expr::cust("60 * 24 * 24"))
            .values(vec![(
                Glyph::Image,
                "24B0E11951B03B07F8300FD003983F03F0780060".into()
            ),])
            .and_where(Expr::col(Glyph::Id).eq(1))
            .returning(Query::returning().columns([Glyph::Id, Glyph::Image]))
            .to_string(SqliteQueryBuilder),
        r#"UPDATE "glyph" SET "aspect" = 60 * 24 * 24, "image" = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE "id" = 1 RETURNING "id", "image""#
    );
}

#[test]
fn delete_1() {
    assert_eq!(
        Query::delete()
            .from_table(Glyph::Table)
            .and_where(Expr::col(Glyph::Id).eq(1))
            .to_string(SqliteQueryBuilder),
        r#"DELETE FROM "glyph" WHERE "id" = 1"#
    );
}

#[test]
fn escape_1() {
    let test = r#" "abc" "#;
    assert_eq!(
        SqliteQueryBuilder.escape_string(test),
        r#" "abc" "#.to_owned()
    );
    assert_eq!(
        SqliteQueryBuilder.unescape_string(SqliteQueryBuilder.escape_string(test).as_str()),
        test
    )
}

#[test]
fn escape_2() {
    let test = "a\nb\tc";
    assert_eq!(SqliteQueryBuilder.escape_string(test), "a\nb\tc".to_owned());
    assert_eq!(
        SqliteQueryBuilder.unescape_string(SqliteQueryBuilder.escape_string(test).as_str()),
        test
    );
}

#[test]
fn escape_3() {
    let test = "a\\b";
    assert_eq!(SqliteQueryBuilder.escape_string(test), "a\\b".to_owned());
    println!("{}", SqliteQueryBuilder.escape_string(test));
    println!(
        "{}",
        SqliteQueryBuilder.unescape_string(SqliteQueryBuilder.escape_string(test).as_str())
    );
    assert_eq!(
        SqliteQueryBuilder.unescape_string(SqliteQueryBuilder.escape_string(test).as_str()),
        test
    );
}

#[test]
fn escape_4() {
    let test = "a\"b";
    assert_eq!(SqliteQueryBuilder.escape_string(test), "a\"b".to_owned());
    assert_eq!(
        SqliteQueryBuilder.unescape_string(SqliteQueryBuilder.escape_string(test).as_str()),
        test
    );
}

#[test]
fn escape_5() {
    let test = "a'c";
    assert_eq!(SqliteQueryBuilder.escape_string(test), "a''c".to_owned());
    assert_eq!(
        SqliteQueryBuilder.unescape_string(SqliteQueryBuilder.escape_string(test).as_str()),
        test
    );
}

#[test]
fn delete_returning_all_columns() {
    assert_eq!(
        Query::delete()
            .from_table(Glyph::Table)
            .and_where(Expr::col(Glyph::Id).eq(1))
            .returning(Query::returning().all())
            .to_string(SqliteQueryBuilder),
        r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING *"#
    );
}

#[test]
fn delete_returning_specific_columns() {
    assert_eq!(
        Query::delete()
            .from_table(Glyph::Table)
            .and_where(Expr::col(Glyph::Id).eq(1))
            .returning(Query::returning().columns([Glyph::Id, Glyph::Image]))
            .to_string(SqliteQueryBuilder),
        r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING "id", "image""#
    );
}

#[test]
fn delete_returning_specific_exprs() {
    assert_eq!(
        Query::delete()
            .from_table(Glyph::Table)
            .and_where(Expr::col(Glyph::Id).eq(1))
            .returning(Query::returning().exprs([Expr::col(Glyph::Id), Expr::col(Glyph::Image)]))
            .to_string(SqliteQueryBuilder),
        r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING "id", "image""#
    );
}
