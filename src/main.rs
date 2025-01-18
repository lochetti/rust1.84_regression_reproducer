use diesel::{ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use schema::*;

mod schema;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    let Ok(database_url) = std::env::var("DATABASE_URL") else {
        return;
    };
    let Ok(mut connection) = AsyncPgConnection::establish(&database_url).await else {
        return;
    };

    let a01 = complex_query(&mut connection).await.unwrap();
    // let a02 = query2(&mut connection).await.unwrap();
    // let a03 = query3(&mut connection).await.unwrap();
    // let a04 = query4(&mut connection).await.unwrap();
    // let a05 = query5(&mut connection).await.unwrap();
    // let a06 = query6(&mut connection).await.unwrap();
    // let a07 = query7(&mut connection).await.unwrap();
    // let a08 = query8(&mut connection).await.unwrap();
    // let a09 = query9(&mut connection).await.unwrap();
    // let a10 = query10(&mut connection).await.unwrap();
    // let a11 = query11(&mut connection).await.unwrap();
    // let a12 = query12(&mut connection).await.unwrap();
    // let a13 = query13(&mut connection).await.unwrap();
    // let a14 = query14(&mut connection).await.unwrap();
    // let a15 = query15(&mut connection).await.unwrap();
    // let a16 = query16(&mut connection).await.unwrap();
    // let a17 = query17(&mut connection).await.unwrap();
    // let a18 = query18(&mut connection).await.unwrap();
    // let a19 = query19(&mut connection).await.unwrap();
    // let a20 = query20(&mut connection).await.unwrap();
    // let a21 = query21(&mut connection).await.unwrap();
    // let a22 = query22(&mut connection).await.unwrap();
    // let a23 = query23(&mut connection).await.unwrap();
    // let a24 = query24(&mut connection).await.unwrap();
    // let a25 = query25(&mut connection).await.unwrap();
    // let a26 = query26(&mut connection).await.unwrap();
    // let a27 = query27(&mut connection).await.unwrap();
    // let a28 = query28(&mut connection).await.unwrap();
    // let a29 = query29(&mut connection).await.unwrap();
    // let a30 = query30(&mut connection).await.unwrap();
    // let a31 = query31(&mut connection).await.unwrap();
    // let a32 = query32(&mut connection).await.unwrap();
    // let a33 = query33(&mut connection).await.unwrap();
    // let a34 = query34(&mut connection).await.unwrap();
    // let a35 = query35(&mut connection).await.unwrap();
    // let a36 = query36(&mut connection).await.unwrap();
    // let a37 = query37(&mut connection).await.unwrap();
    // let a38 = query38(&mut connection).await.unwrap();
    // let a39 = query39(&mut connection).await.unwrap();
    // let a40 = query40(&mut connection).await.unwrap();
    // let a41 = query41(&mut connection).await.unwrap();
    // let a42 = query42(&mut connection).await.unwrap();
    // let a43 = query43(&mut connection).await.unwrap();
    // let a44 = query44(&mut connection).await.unwrap();
    // let a45 = query45(&mut connection).await.unwrap();
    // let a46 = query46(&mut connection).await.unwrap();
    // let a47 = query47(&mut connection).await.unwrap();
    // let a48 = query48(&mut connection).await.unwrap();
    // let a49 = query49(&mut connection).await.unwrap();
    // let a50 = query50(&mut connection).await.unwrap();
    // let a51 = query51(&mut connection).await.unwrap();
    // let a52 = query52(&mut connection).await.unwrap();

    println!("{:?}", a01[0].0);
    // println!("{a02:?}");
    // println!("{a03:?}");
    // println!("{a04:?}");
    // println!("{a05:?}");
    // println!("{a06:?}");
    // println!("{a07:?}");
    // println!("{a08:?}");
    // println!("{a09:?}");
    // println!("{a10:?}");
    // println!("{a11:?}");
    // println!("{a12:?}");
    // println!("{a13:?}");
    // println!("{a14:?}");
    // println!("{a15:?}");
    // println!("{a16:?}");
    // println!("{a17:?}");
    // println!("{a18:?}");
    // println!("{a19:?}");
    // println!("{a20:?}");
    // println!("{a21:?}");
    // println!("{a22:?}");
    // println!("{a23:?}");
    // println!("{a24:?}");
    // println!("{a25:?}");
    // println!("{a26:?}");
    // println!("{a27:?}");
    // println!("{a28:?}");
    // println!("{a29:?}");
    // println!("{a30:?}");
    // println!("{a31:?}");
    // println!("{a32:?}");
    // println!("{a33:?}");
    // println!("{a34:?}");
    // println!("{a35:?}");
    // println!("{a36:?}");
    // println!("{a37:?}");
    // println!("{a38:?}");
    // println!("{a39:?}");
    // println!("{a40:?}");
    // println!("{a41:?}");
    // println!("{a42:?}");
    // println!("{a43:?}");
    // println!("{a44:?}");
    // println!("{a45:?}");
    // println!("{a46:?}");
    // println!("{a47:?}");
    // println!("{a48:?}");
    // println!("{a49:?}");
    // println!("{a50:?}");
    // println!("{a51:?}");
    // println!("{a52:?}");
}

// async fn query1(
//     conn: &mut AsyncPgConnection,
// ) -> QueryResult<
//     Vec<(
//         Option<String>,
//         i32,
//         Option<i32>,
//         Option<String>,
//         i32,
//         Option<String>,
//         Option<i32>,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//     )>,
// > {
//     a::table
//         .inner_join(b::table.on(a::id.nullable().eq(b::a_id)))
//         .left_join(c::table.on(b::id.nullable().eq(c::b_id)))
//         .inner_join(d::table.on(c::id.nullable().eq(d::c_id)))
//         .left_join(e::table.on(d::id.nullable().eq(e::d_id)))
//         .inner_join(f::table.on(e::id.nullable().eq(f::e_id)))
//         .left_join(g::table.on(f::id.nullable().eq(g::f_id)))
//         .inner_join(h::table.on(g::id.nullable().eq(h::g_id)))
//         .left_join(i::table.on(h::id.nullable().eq(i::h_id)))
//         .inner_join(j::table.on(i::id.nullable().eq(j::i_id)))
//         .left_join(k::table.on(j::id.nullable().eq(k::j_id)))
//         .select((
//             a::name.nullable(),
//             b::id.assume_not_null(),
//             c::id.nullable(),
//             d::name.nullable(),
//             e::id.assume_not_null(),
//             f::name.nullable(),
//             g::id.nullable(),
//             h::name.assume_not_null(),
//             i::id.nullable(),
//             j::name.nullable(),
//             k::id.assume_not_null(),
//         ))
//         .get_results(conn)
//         .await
// }

// async fn query2(
//     conn: &mut AsyncPgConnection,
// ) -> QueryResult<
//     Vec<(
//         i32,
//         Option<String>,
//         i32,
//         Option<i32>,
//         String,
//         Option<String>,
//         i32,
//         Option<i32>,
//         String,
//         Option<String>,
//         Option<i32>,
//     )>,
// > {
//     b::table
//         .inner_join(c::table.on(b::id.nullable().eq(c::b_id)))
//         .left_join(d::table.on(c::id.nullable().eq(d::c_id)))
//         .inner_join(e::table.on(d::id.nullable().eq(e::d_id)))
//         .left_join(f::table.on(e::id.nullable().eq(f::e_id)))
//         .inner_join(g::table.on(f::id.nullable().eq(g::f_id)))
//         .left_join(h::table.on(g::id.nullable().eq(h::g_id)))
//         .inner_join(i::table.on(h::id.nullable().eq(i::h_id)))
//         .left_join(j::table.on(i::id.nullable().eq(j::i_id)))
//         .inner_join(k::table.on(j::id.nullable().eq(k::j_id)))
//         .left_join(l::table.on(k::id.nullable().eq(l::k_id)))
//         .select((
//             b::id.assume_not_null(),
//             c::name.nullable(),
//             d::id.assume_not_null(),
//             e::id.nullable(),
//             f::name.assume_not_null(),
//             g::name.nullable(),
//             h::id.assume_not_null(),
//             i::id.nullable(),
//             j::name.assume_not_null(),
//             k::name.nullable(),
//             l::id.nullable(),
//         ))
//         .get_results(conn)
//         .await
// }

// async fn query3(
//     conn: &mut AsyncPgConnection,
// ) -> QueryResult<
//     Vec<(
//         Option<i32>,
//         String,
//         Option<String>,
//         i32,
//         Option<i32>,
//         String,
//         Option<String>,
//         i32,
//         Option<i32>,
//         String,
//         Option<String>,
//     )>,
// > {
//     c::table
//         .inner_join(d::table.on(c::id.nullable().eq(d::c_id)))
//         .left_join(e::table.on(d::id.nullable().eq(e::d_id)))
//         .inner_join(f::table.on(e::id.nullable().eq(f::e_id)))
//         .left_join(g::table.on(f::id.nullable().eq(g::f_id)))
//         .inner_join(h::table.on(g::id.nullable().eq(h::g_id)))
//         .left_join(i::table.on(h::id.nullable().eq(i::h_id)))
//         .inner_join(j::table.on(i::id.nullable().eq(j::i_id)))
//         .left_join(k::table.on(j::id.nullable().eq(k::j_id)))
//         .inner_join(l::table.on(k::id.nullable().eq(l::k_id)))
//         .left_join(m::table.on(l::id.nullable().eq(m::l_id)))
//         .select((
//             c::id.nullable(),
//             d::name.assume_not_null(),
//             e::name.nullable(),
//             f::id.assume_not_null(),
//             g::id.nullable(),
//             h::name.assume_not_null(),
//             i::name.nullable(),
//             j::id.assume_not_null(),
//             k::id.nullable(),
//             l::name.assume_not_null(),
//             m::name.nullable(),
//         ))
//         .get_results(conn)
//         .await
// }

// async fn query4(
//     conn: &mut AsyncPgConnection,
// ) -> QueryResult<
//     Vec<(
//         i32,
//         Option<String>,
//         Option<i32>,
//         String,
//         i32,
//         Option<String>,
//         Option<i32>,
//         String,
//         i32,
//         Option<String>,
//         Option<i32>,
//     )>,
// > {
//     d::table
//         .inner_join(e::table.on(d::id.nullable().eq(e::d_id)))
//         .left_join(f::table.on(e::id.nullable().eq(f::e_id)))
//         .inner_join(g::table.on(f::id.nullable().eq(g::f_id)))
//         .left_join(h::table.on(g::id.nullable().eq(h::g_id)))
//         .inner_join(i::table.on(h::id.nullable().eq(i::h_id)))
//         .left_join(j::table.on(i::id.nullable().eq(j::i_id)))
//         .inner_join(k::table.on(j::id.nullable().eq(k::j_id)))
//         .left_join(l::table.on(k::id.nullable().eq(l::k_id)))
//         .inner_join(m::table.on(l::id.nullable().eq(m::l_id)))
//         .left_join(n::table.on(m::id.nullable().eq(n::m_id)))
//         .select((
//             d::id.assume_not_null(),
//             e::name.nullable(),
//             f::id.nullable(),
//             g::name.assume_not_null(),
//             h::id.assume_not_null(),
//             i::name.nullable(),
//             j::id.nullable(),
//             k::name.assume_not_null(),
//             l::id.assume_not_null(),
//             m::name.nullable(),
//             n::id.nullable(),
//         ))
//         .get_results(conn)
//         .await
// }

// async fn query5(
//     conn: &mut AsyncPgConnection,
// ) -> QueryResult<
//     Vec<(
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         String,
//     )>,
// > {
//     e::table
//         .inner_join(f::table.on(e::id.nullable().eq(f::e_id)))
//         .left_join(g::table.on(f::id.nullable().eq(g::f_id)))
//         .inner_join(h::table.on(g::id.nullable().eq(h::g_id)))
//         .left_join(i::table.on(h::id.nullable().eq(i::h_id)))
//         .inner_join(j::table.on(i::id.nullable().eq(j::i_id)))
//         .left_join(k::table.on(j::id.nullable().eq(k::j_id)))
//         .inner_join(l::table.on(k::id.nullable().eq(l::k_id)))
//         .left_join(m::table.on(l::id.nullable().eq(m::l_id)))
//         .inner_join(n::table.on(m::id.nullable().eq(n::m_id)))
//         .left_join(o::table.on(n::id.nullable().eq(o::n_id)))
//         .select((
//             e::name.nullable(),
//             f::id.assume_not_null(),
//             g::name.assume_not_null(),
//             h::id.nullable(),
//             i::name.nullable(),
//             j::id.assume_not_null(),
//             k::name.assume_not_null(),
//             l::id.nullable(),
//             m::name.nullable(),
//             n::id.assume_not_null(),
//             o::name.assume_not_null(),
//         ))
//         .get_results(conn)
//         .await
// }

// async fn query6(
//     conn: &mut AsyncPgConnection,
// ) -> QueryResult<
//     Vec<(
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//     )>,
// > {
//     f::table
//         .inner_join(g::table.on(f::id.nullable().eq(g::f_id)))
//         .left_join(h::table.on(g::id.nullable().eq(h::g_id)))
//         .inner_join(i::table.on(h::id.nullable().eq(i::h_id)))
//         .left_join(j::table.on(i::id.nullable().eq(j::i_id)))
//         .inner_join(k::table.on(j::id.nullable().eq(k::j_id)))
//         .left_join(l::table.on(k::id.nullable().eq(l::k_id)))
//         .inner_join(m::table.on(l::id.nullable().eq(m::l_id)))
//         .left_join(n::table.on(m::id.nullable().eq(n::m_id)))
//         .inner_join(o::table.on(n::id.nullable().eq(o::n_id)))
//         .left_join(p::table.on(o::id.nullable().eq(p::o_id)))
//         .select((
//             f::id.assume_not_null(),
//             g::name.assume_not_null(),
//             h::id.nullable(),
//             i::name.nullable(),
//             j::id.assume_not_null(),
//             k::name.assume_not_null(),
//             l::id.nullable(),
//             m::name.nullable(),
//             n::id.assume_not_null(),
//             o::name.assume_not_null(),
//             p::id.nullable(),
//         ))
//         .get_results(conn)
//         .await
// }

// async fn query7(
//     conn: &mut AsyncPgConnection,
// ) -> QueryResult<
//     Vec<(
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         Option<String>,
//     )>,
// > {
//     g::table
//         .inner_join(h::table.on(g::id.nullable().eq(h::g_id)))
//         .left_join(i::table.on(h::id.nullable().eq(i::h_id)))
//         .inner_join(j::table.on(i::id.nullable().eq(j::i_id)))
//         .left_join(k::table.on(j::id.nullable().eq(k::j_id)))
//         .inner_join(l::table.on(k::id.nullable().eq(l::k_id)))
//         .left_join(m::table.on(l::id.nullable().eq(m::l_id)))
//         .inner_join(n::table.on(m::id.nullable().eq(n::m_id)))
//         .left_join(o::table.on(n::id.nullable().eq(o::n_id)))
//         .inner_join(p::table.on(o::id.nullable().eq(p::o_id)))
//         .left_join(q::table.on(p::id.nullable().eq(q::p_id)))
//         .select((
//             g::name.nullable(),
//             h::id.assume_not_null(),
//             i::name.assume_not_null(),
//             j::id.nullable(),
//             k::name.nullable(),
//             l::id.assume_not_null(),
//             m::name.assume_not_null(),
//             n::id.nullable(),
//             o::name.nullable(),
//             p::id.assume_not_null(),
//             q::name.nullable(),
//         ))
//         .get_results(conn)
//         .await
// }

// async fn query8(
//     conn: &mut AsyncPgConnection,
// ) -> QueryResult<
//     Vec<(
//         i32,
//         Option<String>,
//         Option<i32>,
//         String,
//         i32,
//         Option<String>,
//         Option<i32>,
//         String,
//         i32,
//         Option<String>,
//         i32,
//     )>,
// > {
//     h::table
//         .inner_join(i::table.on(h::id.nullable().eq(i::h_id)))
//         .left_join(j::table.on(i::id.nullable().eq(j::i_id)))
//         .inner_join(k::table.on(j::id.nullable().eq(k::j_id)))
//         .left_join(l::table.on(k::id.nullable().eq(l::k_id)))
//         .inner_join(m::table.on(l::id.nullable().eq(m::l_id)))
//         .left_join(n::table.on(m::id.nullable().eq(n::m_id)))
//         .inner_join(o::table.on(n::id.nullable().eq(o::n_id)))
//         .left_join(p::table.on(o::id.nullable().eq(p::o_id)))
//         .inner_join(q::table.on(p::id.nullable().eq(q::p_id)))
//         .left_join(r::table.on(q::id.nullable().eq(r::q_id)))
//         .select((
//             h::id.assume_not_null(),
//             i::name.nullable(),
//             j::id.nullable(),
//             k::name.assume_not_null(),
//             l::id.assume_not_null(),
//             m::name.nullable(),
//             n::id.nullable(),
//             o::name.assume_not_null(),
//             p::id.assume_not_null(),
//             q::name.nullable(),
//             r::id.assume_not_null(),
//         ))
//         .get_results(conn)
//         .await
// }

// async fn query9(
//     conn: &mut AsyncPgConnection,
// ) -> QueryResult<
//     Vec<(
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         Option<i32>,
//     )>,
// > {
//     i::table
//         .inner_join(j::table.on(i::id.nullable().eq(j::i_id)))
//         .left_join(k::table.on(j::id.nullable().eq(k::j_id)))
//         .inner_join(l::table.on(k::id.nullable().eq(l::k_id)))
//         .left_join(m::table.on(l::id.nullable().eq(m::l_id)))
//         .inner_join(n::table.on(m::id.nullable().eq(n::m_id)))
//         .left_join(o::table.on(n::id.nullable().eq(o::n_id)))
//         .inner_join(p::table.on(o::id.nullable().eq(p::o_id)))
//         .left_join(q::table.on(p::id.nullable().eq(q::p_id)))
//         .inner_join(r::table.on(q::id.nullable().eq(r::q_id)))
//         .left_join(s::table.on(r::id.nullable().eq(s::r_id)))
//         .select((
//             i::name.nullable(),
//             j::id.assume_not_null(),
//             k::name.assume_not_null(),
//             l::id.nullable(),
//             m::name.nullable(),
//             n::id.assume_not_null(),
//             o::name.assume_not_null(),
//             p::id.nullable(),
//             q::name.nullable(),
//             r::id.assume_not_null(),
//             s::id.nullable(),
//         ))
//         .get_results(conn)
//         .await
// }

// async fn query10(
//     conn: &mut AsyncPgConnection,
// ) -> QueryResult<
//     Vec<(
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         String,
//         Option<String>,
//     )>,
// > {
//     j::table
//         .inner_join(k::table.on(j::id.nullable().eq(k::j_id)))
//         .left_join(l::table.on(k::id.nullable().eq(l::k_id)))
//         .inner_join(m::table.on(l::id.nullable().eq(m::l_id)))
//         .left_join(n::table.on(m::id.nullable().eq(n::m_id)))
//         .inner_join(o::table.on(n::id.nullable().eq(o::n_id)))
//         .left_join(p::table.on(o::id.nullable().eq(p::o_id)))
//         .inner_join(q::table.on(p::id.nullable().eq(q::p_id)))
//         .left_join(r::table.on(q::id.nullable().eq(r::q_id)))
//         .inner_join(s::table.on(r::id.nullable().eq(s::r_id)))
//         .left_join(t::table.on(s::id.nullable().eq(t::s_id)))
//         .select((
//             j::id.assume_not_null(),
//             k::name.assume_not_null(),
//             l::id.nullable(),
//             m::name.nullable(),
//             n::id.assume_not_null(),
//             o::name.assume_not_null(),
//             p::id.nullable(),
//             q::name.nullable(),
//             r::id.assume_not_null(),
//             s::name.assume_not_null(),
//             t::name.nullable(),
//         ))
//         .get_results(conn)
//         .await
// }

// async fn query11(
//     conn: &mut AsyncPgConnection,
// ) -> QueryResult<
//     Vec<(
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         i32,
//     )>,
// > {
//     k::table
//         .inner_join(l::table.on(k::id.nullable().eq(l::k_id)))
//         .left_join(m::table.on(l::id.nullable().eq(m::l_id)))
//         .inner_join(n::table.on(m::id.nullable().eq(n::m_id)))
//         .left_join(o::table.on(n::id.nullable().eq(o::n_id)))
//         .inner_join(p::table.on(o::id.nullable().eq(p::o_id)))
//         .left_join(q::table.on(p::id.nullable().eq(q::p_id)))
//         .inner_join(r::table.on(q::id.nullable().eq(r::q_id)))
//         .left_join(s::table.on(r::id.nullable().eq(s::r_id)))
//         .inner_join(t::table.on(s::id.nullable().eq(t::s_id)))
//         .left_join(u::table.on(t::id.nullable().eq(u::t_id)))
//         .select((
//             k::name.nullable(),
//             l::id.assume_not_null(),
//             m::name.assume_not_null(),
//             n::id.nullable(),
//             o::name.nullable(),
//             p::id.assume_not_null(),
//             q::name.assume_not_null(),
//             r::id.nullable(),
//             s::name.nullable(),
//             t::id.assume_not_null(),
//             u::id.assume_not_null(),
//         ))
//         .get_results(conn)
//         .await
// }

// async fn query12(
//     conn: &mut AsyncPgConnection,
// ) -> QueryResult<
//     Vec<(
//         i32,
//         Option<String>,
//         Option<i32>,
//         String,
//         i32,
//         Option<String>,
//         Option<i32>,
//         String,
//         i32,
//         Option<String>,
//         Option<i32>,
//     )>,
// > {
//     l::table
//         .inner_join(m::table.on(l::id.nullable().eq(m::l_id)))
//         .left_join(n::table.on(m::id.nullable().eq(n::m_id)))
//         .inner_join(o::table.on(n::id.nullable().eq(o::n_id)))
//         .left_join(p::table.on(o::id.nullable().eq(p::o_id)))
//         .inner_join(q::table.on(p::id.nullable().eq(q::p_id)))
//         .left_join(r::table.on(q::id.nullable().eq(r::q_id)))
//         .inner_join(s::table.on(r::id.nullable().eq(s::r_id)))
//         .left_join(t::table.on(s::id.nullable().eq(t::s_id)))
//         .inner_join(u::table.on(t::id.nullable().eq(u::t_id)))
//         .left_join(v::table.on(u::id.nullable().eq(v::u_id)))
//         .select((
//             l::id.assume_not_null(),
//             m::name.nullable(),
//             n::id.nullable(),
//             o::name.assume_not_null(),
//             p::id.assume_not_null(),
//             q::name.nullable(),
//             r::id.nullable(),
//             s::name.assume_not_null(),
//             t::id.assume_not_null(),
//             u::name.nullable(),
//             v::id.nullable(),
//         ))
//         .get_results(conn)
//         .await
// }

// async fn query13(
//     conn: &mut AsyncPgConnection,
// ) -> QueryResult<
//     Vec<(
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         String,
//     )>,
// > {
//     m::table
//         .inner_join(n::table.on(m::id.nullable().eq(n::m_id)))
//         .left_join(o::table.on(n::id.nullable().eq(o::n_id)))
//         .inner_join(p::table.on(o::id.nullable().eq(p::o_id)))
//         .left_join(q::table.on(p::id.nullable().eq(q::p_id)))
//         .inner_join(r::table.on(q::id.nullable().eq(r::q_id)))
//         .left_join(s::table.on(r::id.nullable().eq(s::r_id)))
//         .inner_join(t::table.on(s::id.nullable().eq(t::s_id)))
//         .left_join(u::table.on(t::id.nullable().eq(u::t_id)))
//         .inner_join(v::table.on(u::id.nullable().eq(v::u_id)))
//         .left_join(w::table.on(v::id.nullable().eq(w::v_id)))
//         .select((
//             m::name.nullable(),
//             n::id.assume_not_null(),
//             o::name.assume_not_null(),
//             p::id.nullable(),
//             q::name.nullable(),
//             r::id.assume_not_null(),
//             s::name.assume_not_null(),
//             t::id.nullable(),
//             u::name.nullable(),
//             v::id.assume_not_null(),
//             w::name.assume_not_null(),
//         ))
//         .get_results(conn)
//         .await
// }

// async fn query14(
//     conn: &mut AsyncPgConnection,
// ) -> QueryResult<
//     Vec<(
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//     )>,
// > {
//     n::table
//         .inner_join(o::table.on(n::id.nullable().eq(o::n_id)))
//         .left_join(p::table.on(o::id.nullable().eq(p::o_id)))
//         .inner_join(q::table.on(p::id.nullable().eq(q::p_id)))
//         .left_join(r::table.on(q::id.nullable().eq(r::q_id)))
//         .inner_join(s::table.on(r::id.nullable().eq(s::r_id)))
//         .left_join(t::table.on(s::id.nullable().eq(t::s_id)))
//         .inner_join(u::table.on(t::id.nullable().eq(u::t_id)))
//         .left_join(v::table.on(u::id.nullable().eq(v::u_id)))
//         .inner_join(w::table.on(v::id.nullable().eq(w::v_id)))
//         .left_join(x::table.on(w::id.nullable().eq(x::w_id)))
//         .select((
//             n::id.assume_not_null(),
//             o::name.assume_not_null(),
//             p::id.nullable(),
//             q::name.nullable(),
//             r::id.assume_not_null(),
//             s::name.assume_not_null(),
//             t::id.nullable(),
//             u::name.nullable(),
//             v::id.assume_not_null(),
//             w::name.assume_not_null(),
//             x::id.nullable(),
//         ))
//         .get_results(conn)
//         .await
// }

// async fn query15(
//     conn: &mut AsyncPgConnection,
// ) -> QueryResult<
//     Vec<(
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         Option<String>,
//     )>,
// > {
//     o::table
//         .inner_join(p::table.on(o::id.nullable().eq(p::o_id)))
//         .left_join(q::table.on(p::id.nullable().eq(q::p_id)))
//         .inner_join(r::table.on(q::id.nullable().eq(r::q_id)))
//         .left_join(s::table.on(r::id.nullable().eq(s::r_id)))
//         .inner_join(t::table.on(s::id.nullable().eq(t::s_id)))
//         .left_join(u::table.on(t::id.nullable().eq(u::t_id)))
//         .inner_join(v::table.on(u::id.nullable().eq(v::u_id)))
//         .left_join(w::table.on(v::id.nullable().eq(w::v_id)))
//         .inner_join(x::table.on(w::id.nullable().eq(x::w_id)))
//         .left_join(y::table.on(x::id.nullable().eq(y::x_id)))
//         .select((
//             o::name.nullable(),
//             p::id.assume_not_null(),
//             q::name.assume_not_null(),
//             r::id.nullable(),
//             s::name.nullable(),
//             t::id.assume_not_null(),
//             u::name.assume_not_null(),
//             v::id.nullable(),
//             w::name.nullable(),
//             x::id.assume_not_null(),
//             y::name.nullable(),
//         ))
//         .get_results(conn)
//         .await
// }

// async fn query16(
//     conn: &mut AsyncPgConnection,
// ) -> QueryResult<
//     Vec<(
//         i32,
//         Option<String>,
//         Option<i32>,
//         String,
//         i32,
//         Option<String>,
//         Option<i32>,
//         String,
//         i32,
//         Option<String>,
//         i32,
//     )>,
// > {
//     p::table
//         .inner_join(q::table.on(p::id.nullable().eq(q::p_id)))
//         .left_join(r::table.on(q::id.nullable().eq(r::q_id)))
//         .inner_join(s::table.on(r::id.nullable().eq(s::r_id)))
//         .left_join(t::table.on(s::id.nullable().eq(t::s_id)))
//         .inner_join(u::table.on(t::id.nullable().eq(u::t_id)))
//         .left_join(v::table.on(u::id.nullable().eq(v::u_id)))
//         .inner_join(w::table.on(v::id.nullable().eq(w::v_id)))
//         .left_join(x::table.on(w::id.nullable().eq(x::w_id)))
//         .inner_join(y::table.on(x::id.nullable().eq(y::x_id)))
//         .left_join(z::table.on(y::id.nullable().eq(z::y_id)))
//         .select((
//             p::id.assume_not_null(),
//             q::name.nullable(),
//             r::id.nullable(),
//             s::name.assume_not_null(),
//             t::id.assume_not_null(),
//             u::name.nullable(),
//             v::id.nullable(),
//             w::name.assume_not_null(),
//             x::id.assume_not_null(),
//             y::name.nullable(),
//             z::id.assume_not_null(),
//         ))
//         .get_results(conn)
//         .await
// }

// async fn query17(
//     conn: &mut AsyncPgConnection,
// ) -> QueryResult<
//     Vec<(
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         String,
//         Option<i32>,
//         Option<String>,
//         i32,
//         String,
//     )>,
// > {
//     q::table
//         .inner_join(r::table.on(q::id.nullable().eq(r::q_id)))
//         .left_join(s::table.on(r::id.nullable().eq(s::r_id)))
//         .inner_join(t::table.on(s::id.nullable().eq(t::s_id)))
//         .left_join(u::table.on(t::id.nullable().eq(u::t_id)))
//         .inner_join(v::table.on(u::id.nullable().eq(v::u_id)))
//         .left_join(w::table.on(v::id.nullable().eq(w::v_id)))
//         .inner_join(x::table.on(w::id.nullable().eq(x::w_id)))
//         .left_join(y::table.on(x::id.nullable().eq(y::x_id)))
//         .inner_join(z::table.on(y::id.nullable().eq(z::y_id)))
//         .left_join(a::table.on(z::id.nullable().eq(a::z_id)))
//         .select((
//             q::name.nullable(),
//             r::id.assume_not_null(),
//             s::name.assume_not_null(),
//             t::id.nullable(),
//             u::name.nullable(),
//             v::id.assume_not_null(),
//             w::name.assume_not_null(),
//             x::id.nullable(),
//             y::name.nullable(),
//             z::id.assume_not_null(),
//             a::name.assume_not_null(),
//         ))
//         .get_results(conn)
//         .await
// }
async fn complex_query(
    conn: &mut AsyncPgConnection,
) -> QueryResult<
    Vec<(
        Option<String>,
        i32,
        Option<i32>,
        String,
        Option<String>,
        i32,
        Option<String>,
        i32,
        Option<i32>,
        String,
        Option<String>,
        i32,
        Option<String>,
        i32,
        Option<i32>,
        String,
        Option<String>,
        i32,
        Option<String>,
        i32,
        Option<i32>,
        String,
        Option<String>,
        i32,
        Option<String>,
        i32,
        Option<i32>,
        String,
        Option<String>,
        i32,
    )>,
> {
    a::table
        .inner_join(b::table.on(a::id.nullable().eq(b::a_id)))
        .left_join(c::table.on(b::id.nullable().eq(c::b_id)))
        .inner_join(d::table.on(c::id.nullable().eq(d::c_id)))
        .left_join(e::table.on(d::id.nullable().eq(e::d_id)))
        .inner_join(f::table.on(e::id.nullable().eq(f::e_id)))
        .left_join(g::table.on(f::id.nullable().eq(g::f_id)))
        .inner_join(h::table.on(g::id.nullable().eq(h::g_id)))
        .left_join(i::table.on(h::id.nullable().eq(i::h_id)))
        .inner_join(j::table.on(i::id.nullable().eq(j::i_id)))
        .left_join(k::table.on(j::id.nullable().eq(k::j_id)))
        .inner_join(l::table.on(k::id.nullable().eq(l::k_id)))
        .left_join(m::table.on(l::id.nullable().eq(m::l_id)))
        .inner_join(n::table.on(m::id.nullable().eq(n::m_id)))
        .left_join(o::table.on(n::id.nullable().eq(o::n_id)))
        .inner_join(p::table.on(o::id.nullable().eq(p::o_id)))
        .left_join(q::table.on(p::id.nullable().eq(q::p_id)))
        .inner_join(r::table.on(q::id.nullable().eq(r::q_id)))
        .left_join(s::table.on(r::id.nullable().eq(s::r_id)))
        .inner_join(t::table.on(s::id.nullable().eq(t::s_id)))
        .left_join(u::table.on(t::id.nullable().eq(u::t_id)))
        .inner_join(v::table.on(u::id.nullable().eq(v::u_id)))
        .left_join(w::table.on(v::id.nullable().eq(w::v_id)))
        .inner_join(x::table.on(w::id.nullable().eq(x::w_id)))
        .left_join(y::table.on(x::id.nullable().eq(y::x_id)))
        .inner_join(z::table.on(y::id.nullable().eq(z::y_id)))
        .select((
            a::name.nullable(),
            b::id.assume_not_null(),
            c::id.nullable(),
            d::name.assume_not_null(),
            e::name.nullable(),
            f::id.assume_not_null(),
            g::name.nullable(),
            h::id.assume_not_null(),
            i::id.nullable(),
            j::name.assume_not_null(),
            k::name.nullable(),
            l::id.assume_not_null(),
            m::name.nullable(),
            n::id.assume_not_null(),
            o::id.nullable(),
            p::name.assume_not_null(),
            q::name.nullable(),
            r::id.assume_not_null(),
            s::name.nullable(),
            t::id.assume_not_null(),
            u::id.nullable(),
            v::name.assume_not_null(),
            w::name.nullable(),
            x::id.assume_not_null(),
            y::name.nullable(),
            z::id.assume_not_null(),
            a::id.nullable(),
            b::name.assume_not_null(),
            c::name.nullable(),
            d::id.assume_not_null(),
        ))
        .get_results(conn)
        .await
}
