#[derive(Debug)]
pub struct Cliente {
    pub nome: String,
    pub ano_de_nascimento: u16,
    pub documento: String,
}

impl Cliente {
    pub fn new(nome: String, ano_de_nascimento: u16, documento: String) -> Self {
        Self {
            nome,
            ano_de_nascimento,
            documento,
        }
    }
}

#[cfg(test)]
mod test_cliente {
    use crate::client::Cliente;

    #[test]
    fn cliente_test() {
        let cliente = Cliente::new(String::from("Paulo"), 1990, String::from("Onde?"));
        assert_eq!(cliente.nome, "Paulo");
        assert_eq!(cliente.ano_de_nascimento, 1990);
        assert_eq!(cliente.documento, "Onde?");
    }
}
