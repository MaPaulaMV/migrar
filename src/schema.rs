table! {
    persona (codigo) {
        codigo -> Int4,
        identificacion -> Varchar,
        nombre -> Text,
        genero -> Varchar,
        estadocivil -> Varchar,
        fechanacimiento -> Text,
        telefono -> Varchar,
        direccion -> Text,
        email -> Text,
        validado -> Bool,
        observacion -> Text,
    }
}
