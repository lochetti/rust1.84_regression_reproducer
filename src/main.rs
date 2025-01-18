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
    println!("{:?}", a01[0].0);
}

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
