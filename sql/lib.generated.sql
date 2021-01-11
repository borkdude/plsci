-- ./src/lib.rs:61:0
CREATE OR REPLACE FUNCTION "plsci"("expr" text) RETURNS text STRICT LANGUAGE c AS 'MODULE_PATHNAME', 'plsci_wrapper';
