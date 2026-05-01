use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use std::fs;
use tera::{Context, Tera};

// --- BLOCO DE ESTRUTURAS ÚNICO (PRESERVAÇÃO E LIMPEZA) ---

#[derive(Serialize, Deserialize, Clone)]
struct Produto {
    id: String,
    nome: String,
    categoria: String,
    imagem: String,
    preco: f64,
    descricao: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ItemCarrinho {
    pub id: u32,
    pub nome: String,
    pub preco: f64,
}

#[derive(Deserialize)]
pub struct DadosLogin {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]


#[allow(dead_code)]
struct DadosCadastro {
    nome: String,
    email: String,
    cep: String,
    endereco: String,
    password: String,
}

// --- FUNÇÕES DE PROCESSAMENTO (HANDLERS) ---

async fn validar_login(form: web::Form<DadosLogin>) -> impl Responder {
    let email_digitado = form.email.to_lowercase();
    let senha_digitada = &form.password;

    println!("Monitoramento: Tentativa de login com {}", email_digitado);

    if (email_digitado == "user@teste.com" || email_digitado == "user@reste.com")
        && senha_digitada == "123456"
    {
        println!("Sucesso: Credenciais válidas.");
        HttpResponse::SeeOther()
            .append_header(("Location", "/checkout"))
            .finish()
    } else {
        HttpResponse::Ok().body("Login ou Senha incorretos. Por favor, tente novamente.")
    }
}

async fn processar_cadastro(form: web::Form<DadosCadastro>) -> impl Responder {
    println!("--- Novo Cadastro Recebido: {} ---", form.nome);
    HttpResponse::SeeOther()
        .append_header(("Location", "/checkout"))
        .finish()
}

// --- LÓGICA DO CARRINHO ---

async fn adicionar_ao_carrinho(path: web::Path<u32>) -> impl Responder {
    let id_produto = path.into_inner();
    println!("Log: Adicionando item ID {} ao sistema", id_produto);

    // Futura integração com SQLx/SQLite aqui

    HttpResponse::SeeOther()
        .append_header(("Location", "/checkout"))
        .finish()
}

async fn remover_item_carrinho(path: web::Path<u32>) -> impl Responder {
    let id_produto = path.into_inner();
    println!("Log: Removendo item ID {} do carrinho", id_produto);

    HttpResponse::SeeOther()
        .append_header(("Location", "/checkout"))
        .finish()
}

// --- TELAS E RENDERIZAÇÃO ---

async fn exibir_vitrine(tmpl: web::Data<Tera>) -> impl Responder {
    let data = fs::read_to_string("produtos.json").expect("Erro ao ler produtos.json");
    let produtos: Vec<Produto> = serde_json::from_str(&data).unwrap();

    let mut ctx = Context::new();
    ctx.insert("produtos", &produtos);

    let rendered = tmpl.render("vitrine.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

async fn tela_login(tmpl: web::Data<Tera>) -> impl Responder {
    let rendered = tmpl.render("login.html", &Context::new()).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

async fn tela_cadastro(tmpl: web::Data<Tera>) -> impl Responder {
    let rendered = tmpl.render("cadastro.html", &Context::new()).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

async fn tela_checkout(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();

    // Mock de dados para teste do layout
    context.insert("produto_nome", "Equipamento de Teste");
    context.insert("endereco_usuario", "Rua de Exemplo, 123");
    context.insert("cep_usuario", "00000-000");
    context.insert("frete_valor", "25.00");
    context.insert("total_pedido", "150.00");

    match tera.render("checkout.html", &context) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao carregar checkout"),
    }
}

// --- CONFIGURAÇÃO DO SERVIDOR ---

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").expect("Erro ao carregar templates");
    let data_tera = web::Data::new(tera);

    println!("Servidor rodando em http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(data_tera.clone())
            .route("/", web::get().to(exibir_vitrine))
            .route("/login", web::get().to(tela_login))
            .route("/login", web::post().to(validar_login))
            .route("/cadastro", web::get().to(tela_cadastro))
            .route("/cadastro", web::post().to(processar_cadastro))
            .route("/checkout", web::get().to(tela_checkout))
            // Novas rotas de ação do carrinho
            .route("/adicionar/{id}", web::get().to(adicionar_ao_carrinho))
            .route("/remover/{id}", web::get().to(remover_item_carrinho))
            // Arquivos estáticos
            .service(actix_files::Files::new("/static", "static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
