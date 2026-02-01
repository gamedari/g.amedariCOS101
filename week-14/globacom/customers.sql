--
-- PostgreSQL database dump
--

\restrict g45HmSesBLUdOqnhjnXcW4mJpHgcxJpBysbjFUfRQwY7oMdMfa8wVVgYz8joTbG

-- Dumped from database version 18.1
-- Dumped by pg_dump version 18.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customers; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customers (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age integer NOT NULL,
    c_email text NOT NULL,
    c_mobile bigint NOT NULL,
    eid integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customers OWNER TO postgres;

--
-- Data for Name: customers; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customers (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	8055089112	102	5
111	Lilian jaiya	43	l_jaiye@gmail.com	8055185341	100	3
112	Arthur Musa	50	a_musa@gmail.com	7055282813	107	10
113	Philip Akonjo	41	p_akonjo@gmail.com	9052356772	100	2
114	Marylene Mapa	33	m_mapa@gmail.com	8053333551	120	5
115	Oghenero Agor	50	o_agor@gmail.com	7055566774	117	11
116	Adams Bree	33	a_bree@gmail.com	8056765424	102	1
117	Okafor Mathias	45	o_mathias@gmail.com	8056763367	120	10
118	Samson Adeleke	65	s_adeleke@gmail.com	7056774423	117	11
119	Lawal Tamire	35	l_tamire@gmail.com	9052111101	107	5
120	James Job	44	j_job@gmail.com	8059693919	100	8
121	Matthew Jakande	21	m_jakande@gmail.com	7051232144	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com	8054921923	107	5
\.


--
-- Name: customers customers_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customers
    ADD CONSTRAINT customers_pkey PRIMARY KEY (c_id);


--
-- PostgreSQL database dump complete
--

\unrestrict g45HmSesBLUdOqnhjnXcW4mJpHgcxJpBysbjFUfRQwY7oMdMfa8wVVgYz8joTbG

