#[derive(Clone)]
struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: (Permissao, Permissao, Permissao),
    usuario: Usuario,
    grupo: Grupo,
}
#[derive(Clone)]
struct Permissao {
    leitura: bool,
    escrita: bool,
    execucao: bool,
}
#[derive(Clone)]
struct Diretorio {
    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: (Permissao, Permissao, Permissao),
    dono: Usuario,
}
#[derive(Clone)]
struct Usuario {
    nome: String,
    uid: u16,
    grupo: Grupo,
}
#[derive(Clone)]
struct Grupo {
    nome: String,
    gid: u16,
    membros: Vec<Usuario>,
}
impl Arquivo{
    fn new(nome: String, tamanho: u64, usuario: Usuario, grupo: Grupo) -> Arquivo {
        Arquivo{
            nome,
            tamanho,
            permissoes: (Permissao {
                leitura: false,
                escrita: true,
                execucao: false,
            }, Permissao {
                leitura: false,
                escrita: true,
                execucao: false,
            }, Permissao {
                leitura: false,
                escrita: true,
                execucao: false,
            }),
            usuario,
            grupo,
        }

    }
    fn alterar_permissoes(&mut self, permi: (Permissao, Permissao, Permissao)){
        self.permissoes = permi;
    }
    fn stat(&self) {
        println!("Arquivo: {}", self.nome);
        println!("Tamanho: {}", self.tamanho);
        println!(
            "Permissões: {}/{}{}{}", format!("{}{}{}", self.permissoes.0.octal(), self.permissoes.1.octal(), self.permissoes.2.octal()),
            self.permissoes.0.rwx(), self.permissoes.1.rwx(), self.permissoes.2.rwx());
        println!("Uid: {}", self.usuario.uid);
        println!("Gid: {}", self.usuario.grupo.gid);
    }
}
impl Permissao {
    fn new(leitura: bool, escrita: bool, execucao: bool) -> Permissao {
        Permissao {leitura, escrita, execucao}
    }
    fn octal(&self) -> u8 {
        let a: u8 = self.leitura as u8;
        let b: u8 = self.escrita as u8;
        let c: u8 = self.execucao as u8;

        match (a, b, c) {
            (0, 0, 0) => 0,
            (0, 0, 1) => 1,
            (0, 1, 0) => 2,
            (0, 1, 1) => 3,
            (1, 0, 0) => 4,
            (1, 0, 1) => 5,
            (1, 1, 0) => 6,
            (1, 1, 1) => 7,
            _ => 0,
        }
    }
    fn rwx(&self) -> String{
        let leitura = if self.leitura {'r'} else {'-'};
        let escrita = if self.escrita  {'w'} else {'-'};
        let execucao = if self.execucao  {'x'} else {'-'};

        format!("{}{}{}", leitura, escrita, execucao)
    }
}
impl Diretorio {
    fn new(nome: String, permissao: (Permissao, Permissao, Permissao), dono: Usuario) -> Diretorio {
        Diretorio{
            nome,
            arquivos: Vec::new(),
            permissoes: permissao,
            dono,
        }
    }
}
impl Usuario {
    fn new(nome: String, uid: u16, grupo: Grupo) -> Usuario {
        Usuario{nome, uid, grupo}
    }
    fn adiciona_grupo(&mut self, grupo: Grupo) {
        self.grupo = grupo;
    }

    fn listar_grupos(&self) {
        println!("Grupo: {}", self.grupo.nome);
    }
}
impl Grupo{
    fn new(nome: String, gid: u16) -> Grupo {
        Grupo{
            nome,
            gid,
            membros: Vec::new(),
        }
    }
    fn adiciona_membro(&mut self, usuario: Usuario) {
        self.membros.push(usuario);
    }
    fn remove_membro(&mut self, usuario: &Usuario) {
        self.membros.retain(|m| m.uid != usuario.uid);
    }
    fn listar_membro(&self) {
        for membro in &self.membros {
            println!("Membro: {}", membro.nome);
        }
    }
}
fn criar_arquivo(arquivos: &mut Vec<Arquivo>, usuarios: &Vec<Usuario>, grupos: &Vec<Grupo>, diretorios: &mut Vec<String>) {
    let mut nome: String = String::new();
    println!("Digite o nome do arquivo a ser criado:");
    std::io::stdin().read_line(&mut nome).expect("Erro ao ler linha!");
    let nome = nome.trim().to_string();

    let mut uid_input: String = String::new();
    println!("Digite o UID do dono do arquivo:");
    std::io::stdin().read_line(&mut uid_input).expect("Erro ao ler linha!");
    let uid: u16 = uid_input.trim().parse().expect("Erro ao converter UID!");

    let usuario = usuarios.iter().find(|u| u.uid == uid).expect("Usuário não encontrado.");

    let mut gid_input: String = String::new();
    println!("Digite o GID do grupo do arquivo:");
    std::io::stdin().read_line(&mut gid_input).expect("Erro ao ler linha!");
    let gid: u16 = gid_input.trim().parse().expect("Erro ao converter GID!");

    let grupo = grupos.iter().find(|g| g.gid == gid).expect("Grupo não encontrado.");

    let mut nome_diretorio: String = String::new();
    println!("Digite o nome do diretório onde o arquivo será criado:");
    std::io::stdin().read_line(&mut nome_diretorio).expect("Erro ao ler linha!");
    let nome_diretorio = nome_diretorio.trim();

    if !diretorios.contains(&nome_diretorio.to_string()) {
        criar_diretorio(diretorios, nome_diretorio);
    }

    let novo_arquivo = Arquivo::new(nome.clone(), 0, usuario.clone(), grupo.clone());
    arquivos.push(novo_arquivo);
    println!("Arquivo '{}' criado no diretório '{}'.", nome, nome_diretorio);
}
fn editar_arquivo(arquivos: &mut Vec<Arquivo>, pos: usize, novo_nome: String) {

    if pos < arquivos.len() {

        let arquivo_existente = &arquivos[pos];


        let novo_arquivo = Arquivo {

            nome: novo_nome.clone(),
            tamanho: arquivo_existente.tamanho,
            permissoes: arquivo_existente.permissoes.clone(),
            usuario: arquivo_existente.usuario.clone(),
            grupo: arquivo_existente.grupo.clone(),

        };

        arquivos[pos] = novo_arquivo;
        println!("Arquivo editado com sucesso para '{}'.", novo_nome);

    } else {

        println!("Posição inválida.");

    }
}
fn apagar_arquivo(arquivos: &mut Vec<Arquivo>) {

    let mut nome: String = String::new();
    println!("Digite o nome do arquivo a ser apagado:");
    std::io::stdin().read_line(&mut nome).expect("Erro ao ler linha!");
    let nome = nome.trim();

    if let Some(pos) = arquivos.iter().position(|x| x.nome == nome) {
        arquivos.remove(pos);
        println!("Arquivo '{}' apagado.", nome);

    } else {
        println!("Arquivo '{}' não encontrado.", nome);
    }
}
fn listar_arquivos(arquivos: &Vec<Arquivo>) {

    if arquivos.is_empty() {
        println!("Nenhum arquivo encontrado.");

    } else {
        println!("Arquivos:");

        for arquivo in arquivos {
            arquivo.stat();
        }
    }
}
fn criar_diretorio(diretorios: &mut Vec<String>, nome: &str) {

    if diretorios.contains(&nome.to_string()) {
        println!("Diretório '{}' já existe.", nome);

    } else {
        diretorios.push(nome.to_string());
        println!("Diretório '{}' criado.", nome);

    }
}
fn editar_diretorio(diretorios: &mut Vec<String>, nome_antigo: &str, nome_novo: &str) {

    if let Some(pos) = diretorios.iter().position(|x| *x == nome_antigo) {
        diretorios[pos] = nome_novo.to_string();
        println!("Diretório '{}' renomeado para '{}'.", nome_antigo, nome_novo);

    } else {
        println!("Diretório '{}' não encontrado.", nome_antigo);

    }
}
fn apagar_diretorio(diretorios: &mut Vec<String>, nome: &str) {
    if let Some(pos) = diretorios.iter().position(|x| *x == nome) {
        diretorios.remove(pos);
        println!("Diretório '{}' apagado.", nome);
    } else {
        println!("Diretório '{}' não encontrado.", nome);
    }
}
fn listar_diretorios(diretorios: &Vec<String>) {
    println!("Subdiretórios:");
    for nome in diretorios {
        println!("{}", nome);
    }
}
fn criar_usuario(usuarios: &mut Vec<Usuario>) {
    let mut nome: String = String::new();
    let mut uid_input: String = String::new();
    let mut grupo_nome: String = String::new();

    println!("Digite o nome do usuário:");
    std::io::stdin().read_line(&mut nome).expect("Erro ao ler linha!");
    let nome = nome.trim().to_string();

    println!("Digite o UID do usuário:");
    std::io::stdin().read_line(&mut uid_input).expect("Erro ao ler linha!");
    let uid: u16 = uid_input.trim().parse().expect("Erro ao converter UID!");

    println!("Digite o nome do grupo do usuário:");
    std::io::stdin().read_line(&mut grupo_nome).expect("Erro ao ler linha!");
    let grupo_nome = grupo_nome.trim().to_string();

    let grupo = Grupo::new(grupo_nome.clone(), uid);

    let usuario = Usuario::new(nome.clone(), uid, grupo);
    usuarios.push(usuario);

    println!("Usuário '{}' criado com UID {} e grupo '{}'.", nome, uid, grupo_nome);
}
fn editar_usuario(usuarios: &mut Vec<Usuario>) {
    let mut uid_input: String = String::new();
    println!("Digite o UID do usuário que deseja editar:");
    std::io::stdin().read_line(&mut uid_input).expect("Erro ao ler linha!");
    let uid: u16 = uid_input.trim().parse().expect("Erro ao converter UID!");

    if let Some(usuario) = usuarios.iter_mut().find(|u| u.uid == uid) {
        let mut novo_nome: String = String::new();
        println!("Digite o novo nome do usuário (deixe em branco para não alterar):");
        std::io::stdin().read_line(&mut novo_nome).expect("Erro ao ler linha!");
        let novo_nome = novo_nome.trim();

        if !novo_nome.is_empty() {
            usuario.nome = novo_nome.to_string();
        }

        let mut novo_grupo_nome: String = String::new();
        println!("Digite o novo grupo do usuário (deixe em branco para não alterar):");
        std::io::stdin().read_line(&mut novo_grupo_nome).expect("Erro ao ler linha!");
        let novo_grupo_nome = novo_grupo_nome.trim();

        if !novo_grupo_nome.is_empty() {
            usuario.grupo.nome = novo_grupo_nome.to_string();
        }

        println!("Usuário editado com sucesso.");
    } else {
        println!("Usuário com UID {} não encontrado.", uid);
    }
}
fn apagar_usuario(usuarios: &mut Vec<Usuario>) {
    let mut uid_input: String = String::new();
    println!("Digite o UID do usuário que deseja apagar:");
    std::io::stdin().read_line(&mut uid_input).expect("Erro ao ler linha!");
    let uid: u16 = uid_input.trim().parse().expect("Erro ao converter UID!");

    if let Some(pos) = usuarios.iter().position(|u| u.uid == uid) {
        usuarios.remove(pos);
        println!("Usuário com UID {} apagado.", uid);
    } else {
        println!("Usuário com UID {} não encontrado.", uid);
    }
}
fn listar_usuarios(usuarios: &Vec<Usuario>) {
    if usuarios.is_empty() {
        println!("Nenhum usuário encontrado.");
    } else {
        println!("Usuários:");
        for usuario in usuarios {
            println!("Nome: {}, UID: {}", usuario.nome, usuario.uid);
            println!("Grupo: {}", usuario.grupo.nome);
        }
    }
}
fn criar_grupo(grupos: &mut Vec<Grupo>) {
    let mut nome: String = String::new();
    let mut gid_input: String = String::new();

    println!("Digite o nome do grupo:");
    std::io::stdin().read_line(&mut nome).expect("Erro ao ler linha!");
    let nome = nome.trim().to_string();

    println!("Digite o GID do grupo:");
    std::io::stdin().read_line(&mut gid_input).expect("Erro ao ler linha!");
    let gid: u16 = gid_input.trim().parse().expect("Erro ao converter GID!");

    let grupo = Grupo::new(nome.clone(), gid);
    grupos.push(grupo);

    println!("Grupo '{}' criado com GID {}.", nome, gid);
}
fn editar_grupo(grupos: &mut Vec<Grupo>) {
    let mut gid_input: String = String::new();
    println!("Digite o GID do grupo que deseja editar:");
    std::io::stdin().read_line(&mut gid_input).expect("Erro ao ler linha!");
    let gid: u16 = gid_input.trim().parse().expect("Erro ao converter GID!");

    if let Some(grupo) = grupos.iter_mut().find(|g| g.gid == gid) {
        let mut novo_nome: String = String::new();
        println!("Digite o novo nome do grupo (deixe em branco para não alterar):");
        std::io::stdin().read_line(&mut novo_nome).expect("Erro ao ler linha!");
        let novo_nome = novo_nome.trim();

        if !novo_nome.is_empty() {
            grupo.nome = novo_nome.to_string();
        }

        let mut novo_gid_input: String = String::new();
        println!("Digite o novo GID do grupo (deixe em branco para não alterar):");
        std::io::stdin().read_line(&mut novo_gid_input).expect("Erro ao ler linha!");
        let novo_gid = novo_gid_input.trim();

        if !novo_gid.is_empty() {
            grupo.gid = novo_gid.parse().expect("Erro ao converter novo GID.");
        }

        println!("Grupo editado com sucesso.");
    } else {
        println!("Grupo com GID {} não encontrado.", gid);
    }
}
fn apagar_grupo(grupos: &mut Vec<Grupo>) {
    let mut gid_input: String = String::new();
    println!("Digite o GID do grupo que deseja apagar:");
    std::io::stdin().read_line(&mut gid_input).expect("Erro ao ler linha!");
    let gid: u16 = gid_input.trim().parse().expect("Erro ao converter GID!");

    if let Some(pos) = grupos.iter().position(|g| g.gid == gid) {
        grupos.remove(pos);
        println!("Grupo com GID {} apagado.", gid);
    } else {
        println!("Grupo com GID {} não encontrado.", gid);
    }
}
fn listar_grupos(grupos: &Vec<Grupo>) {
    if grupos.is_empty() {
        println!("Nenhum grupo encontrado.");
    } else {
        println!("Grupos:");
        for grupo in grupos {
            println!("Nome: {}, GID: {}", grupo.nome, grupo.gid);
        }
    }
}

fn main() {
    let mut arquivos: Vec<Arquivo> = Vec::new();
    let mut diretorios: Vec<String> = Vec::new();
    let mut usuarios: Vec<Usuario> = Vec::new();
    let mut grupos: Vec<Grupo> = Vec::new();

    loop {
        println!("1- Criar Arquivo");
        println!("2- Editar Arquivo");
        println!("3- Apagar Arquivo");
        println!("4- Listar Arquivo");
        println!("5- Criar Diretorio");
        println!("6- Editar Diretorio");
        println!("7- Apagar Diretorio");
        println!("8- Listar Diretorios");
        println!("9- Criar Usuário");
        println!("10- Editar Usuário");
        println!("11- Apagar Usuário");
        println!("12- Listar Usuários");
        println!("13- Criar Grupo");
        println!("14- Editar Grupo");
        println!("15- Apagar Grupo");
        println!("16- Listar Grupos");
        println!("0- Sair");

        let mut selecao: String = String::new();
        std::io::stdin().read_line(&mut selecao).expect("Erro ao ler linha!");
        let escolha: i32 = selecao.trim().parse().expect("Erro ao converter!");

        match escolha {
            1 => criar_arquivo(&mut arquivos, &usuarios, &grupos, &mut diretorios),
            2 => {
                println!("Digite o posição(no índice) do arquivo a ser editado (0 a {}):", arquivos.len() - 1);
                let mut pos_input = String::new();
                std::io::stdin().read_line(&mut pos_input).expect("Falha ao ler a entrada");
                let pos: usize = pos_input.trim().parse().expect("Por favor, insira um número válido");

                println!("Digite o novo nome do arquivo:");

                let mut novo_nome = String::new();
                std::io::stdin().read_line(&mut novo_nome).expect("Falha ao ler a entrada");
                let novo_nome = novo_nome.trim().to_string();
                editar_arquivo(&mut arquivos, pos, novo_nome);
            },
            3 => apagar_arquivo(&mut arquivos),
            4 => listar_arquivos(&arquivos),
            5 => {
                let mut nome_diretorio: String = String::new();
                println!("Digite o nome do diretório a ser criado:");
                std::io::stdin().read_line(&mut nome_diretorio).expect("Erro ao ler linha!");
                let nome_diretorio = nome_diretorio.trim();
                criar_diretorio(&mut diretorios, nome_diretorio);
            },
            6 => {
                let mut nome_antigo: String = String::new();
                println!("Digite o nome do diretório que deseja editar:");
                std::io::stdin().read_line(&mut nome_antigo).expect("Erro ao ler linha!");
                let nome_antigo = nome_antigo.trim();
                let mut nome_novo: String = String::new();
                println!("Digite o novo nome do diretório:");
                std::io::stdin().read_line(&mut nome_novo).expect("Erro ao ler linha!");
                let nome_novo = nome_novo.trim();
                editar_diretorio(&mut diretorios, nome_antigo, nome_novo);
            },
            7 => {
                let mut nome_diretorio: String = String::new();
                println!("Digite o nome do diretório que deseja apagar:");
                std::io::stdin().read_line(&mut nome_diretorio).expect("Erro ao ler linha!");
                let nome_diretorio = nome_diretorio.trim();
                apagar_diretorio(&mut diretorios, nome_diretorio);
            },
            8 => listar_diretorios(&diretorios),
            9 => criar_usuario(&mut usuarios),
            10 => editar_usuario(&mut usuarios),
            11 => apagar_usuario(&mut usuarios),
            12 => listar_usuarios(&usuarios),
            13 => criar_grupo(&mut grupos),
            14 => editar_grupo(&mut grupos),
            15 => apagar_grupo(&mut grupos),
            16 => listar_grupos(&grupos),
            0 => {
                println!("Saindo...");
                break;
            },
            _ => println!("Opção Inválida"),
        }
    }
}