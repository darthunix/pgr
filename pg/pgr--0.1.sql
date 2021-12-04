-- complain if script is sourced in psql
-- rather than via "create extension"
\echo use "create extension pgr" to load this file. \quit

create function hello_rust() returns text
as '$libdir/pgr', 'hello_rust' language C;
