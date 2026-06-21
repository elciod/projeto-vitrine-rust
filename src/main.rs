use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use std::fs;
use tera::{Context, Tera};

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
pub struct CadastroForm {
    pub nome: String,
    pub email: String,

    pub password: String,
    pub password_confirm: String,

    pub cep: String,
    pub rua: String,
    pub numero: String,
    pub complemento: Option<String>,

    pub referencia: Option<String>,

    pub cidade: String,
    pub estado: String,
}

//-----------------------------------------------------------------

#[derive(serde::Serialize)]
#[allow(dead_code)]
struct AlertaContext {
    mensagem: String,
}

async fn validar_login(form: web::Form<DadosLogin>, tmpl: web::Data<tera::Tera>) -> impl Responder {
    let email_valido = "tom_email@teste.com";
    let senha_valida = "123456";

    let email_digitado = form.email.trim().to_lowercase();

    if email_digitado == email_valido && form.password == senha_valida {
        println!("Login realizado com sucesso para: {}", email_digitado);

        HttpResponse::Found()
            .append_header(("Location", "/checkout")) //dashboard
            .finish()
    } else {
        println!("Falha: Usuário ou senha não coincidem.");

        let mut ctx = tera::Context::new();
        ctx.insert("erro", "E-mail ou senha não cadastrados ou incorretos.");

        match tmpl.render("login.html", &ctx) {
            Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
            Err(_) => HttpResponse::InternalServerError().body("Erro no servidor"),
        }
    }
}

async fn processar_cadastro(
    form: web::Form<CadastroForm>,
    _tmpl: web::Data<tera::Tera>,
) -> impl Responder {
    println!("--- Novo Cadastro Recebido ---");
    println!("Usuário: {}", form.nome);
    println!("E-mail: {}", form.email);
    println!(
        "Endereço: {}, Nº {}, CEP: {}",
        form.rua, form.numero, form.cep
    );

    println!("Referência: {:?}", form.referencia);

    println!("Localidade: {}/{}", form.cidade, form.estado);

    println!("------------------------------");

    let mut ctx = tera::Context::new();
    ctx.insert(
        "mensagem_sucesso",
        &format!("Cadastro de {} realizado com sucesso!", form.nome),
    );

    actix_web::HttpResponse::Found()
        .append_header(("Location", "/login"))
        .finish()
}

async fn adicionar_ao_carrinho(path: web::Path<u32>) -> impl Responder {
    let id_produto = path.into_inner();
    println!("Log: Adicionando item ID {} ao sistema", id_produto);

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

async fn tela_cadastro(tmpl: web::Data<tera::Tera>) -> impl Responder {
    let ctx = tera::Context::new(); // Prepara o terreno para mensagens de erro

    match tmpl.render("cadastro.html", &ctx) {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(e) => {
            println!("Erro ao carregar cadastro.html: {}", e);
            HttpResponse::InternalServerError().body("Erro ao carregar a página")
        }
    }
}

async fn tela_checkout(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();

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
            .route("/adicionar/{id}", web::get().to(adicionar_ao_carrinho))
            .route("/remover/{id}", web::get().to(remover_item_carrinho))
            .service(actix_files::Files::new("/static", "static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
