use std::io;
use std::collections::HashMap;

fn main() {
    let mut usuarios: Vec<HashMap<String, String>> = Vec::new();
    let mut id = 0;

    loop {
        println!("Escolha uma opção:");
        println!("0 - Encerrar");
        println!("1 - Cadastrar novo usuário");
        println!("2 - Ler usuários cadastrados");
        println!("3 - Deletar usuário por ID");
        println!("4 - Atualizar usuário por ID");

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("Falha ao ler a entrada");
        let escolha: u32 = escolha.trim().parse().expect("Por favor, insira um número válido.");

        match escolha {
            0 => {
                println!("Encerrando o programa.");
                break;
            }
            1 => {
                let mut usuario: HashMap<String, String> = HashMap::new();

                println!("Digite o nome:");
                let mut nome = String::new();
                io::stdin().read_line(&mut nome).expect("Falha ao ler a entrada");
                usuario.insert("nome".to_string(), nome.trim().to_string());

                println!("Digite a idade:");
                let mut idade = String::new();
                io::stdin().read_line(&mut idade).expect("Falha ao ler a entrada");
                usuario.insert("idade".to_string(), idade.trim().to_string());

                println!("Digite o CEP:");
                let mut cep = String::new();
                io::stdin().read_line(&mut cep).expect("Falha ao ler a entrada");
                usuario.insert("cep".to_string(), cep.trim().to_string());

                println!("Digite a data de aniversário (formato: DD-MM-YYYY):");
                let mut niver = String::new();
                io::stdin().read_line(&mut niver).expect("Falha ao ler a entrada");
                usuario.insert("niver".to_string(), niver.trim().to_string());

                // Adiciona o usuário ao vetor com um ID
                usuarios.push(usuario);
                println!("Usuário cadastrado com sucesso. ID: {}", id);

                // Incrementa o ID para o próximo usuário
                id += 1;
            }
            2 => {
                // Exibe a lista de usuários preenchida
                println!("Lista de Usuários:");
                for (i, usuario) in usuarios.iter().enumerate() {
                    println!("[ {}: {:?} ]", i, usuario);
                }
            }
            3 => {
                // Deletar usuário por ID
                println!("Digite o ID do usuário que deseja deletar:");

                let mut id_deletar = String::new();
                io::stdin().read_line(&mut id_deletar).expect("Falha ao ler a entrada");
                let id_deletar: usize = id_deletar.trim().parse().expect("Por favor, insira um número válido.");

                if id_deletar < usuarios.len() {
                    usuarios.remove(id_deletar);
                    println!("Usuário removido com sucesso.");
                } else {
                    println!("ID não existe. Nenhum usuário removido.");
                }
            }
            4 => {
                // Atualizar usuário por ID
                println!("Digite o ID do usuário que deseja atualizar:");

                let mut id_atualizar = String::new();
                io::stdin().read_line(&mut id_atualizar).expect("Falha ao ler a entrada");
                let id_atualizar: usize = id_atualizar.trim().parse().expect("Por favor, insira um número válido.");

                if id_atualizar < usuarios.len() {
                    let mut usuario: HashMap<String, String> = HashMap::new();

                    println!("Digite o novo nome:");
                    let mut nome = String::new();
                    io::stdin().read_line(&mut nome).expect("Falha ao ler a entrada");
                    usuario.insert("nome".to_string(), nome.trim().to_string());

                    println!("Digite a nova idade:");
                    let mut idade = String::new();
                    io::stdin().read_line(&mut idade).expect("Falha ao ler a entrada");
                    usuario.insert("idade".to_string(), idade.trim().to_string());

                    println!("Digite o novo CEP:");
                    let mut cep = String::new();
                    io::stdin().read_line(&mut cep).expect("Falha ao ler a entrada");
                    usuario.insert("cep".to_string(), cep.trim().to_string());

                    println!("Digite a nova data de aniversário (formato: DD-MM-YYYY):");
                    let mut niver = String::new();
                    io::stdin().read_line(&mut niver).expect("Falha ao ler a entrada");
                    usuario.insert("niver".to_string(), niver.trim().to_string());

                    // Atualiza as informações do usuário
                    usuarios[id_atualizar] = usuario;
                    println!("Usuário atualizado com sucesso.");
                } else {
                    println!("ID não existe. Nenhum usuário atualizado.");
                }
            }
            _ => {
                println!("Opção inválida. Por favor, escolha uma opção válida.");
            }
        }
    }

    println!("Programa encerrado.");
}
