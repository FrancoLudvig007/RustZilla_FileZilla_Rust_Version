use iced::widget::{button, column, row, scrollable, text, text_input, Container};
use iced::{executor, Application, Command, Element, Length, Settings, Theme};

pub fn main() -> iced::Result {
    FuleZilla::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1100.0, 750.0),
            min_size: Some(iced::Size::new(900.0, 650.0)),
            ..Default::default()
        },
        ..Default::default()
    })
}

// Representa os estados possíveis e as entradas do usuário
struct FuleZilla {
    // Estado da aba (Cliente ou Servidor)
    active_tab: Tab,
    
    // Dados do Cliente
    client_host: String,
    client_user: String,
    client_pass: String,
    client_port: String,
    client_connected: bool,
    client_log: String,
    
    // Dados do Servidor
    server_port: String,
    server_user: String,
    server_pass: String,
    server_running: bool,
    server_log: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Client,
    Server,
}

// Todas as interações da interface disparam uma Message
#[derive(Debug, Clone)]
enum Message {
    TabSelected(Tab),
    // Atualizações de Input (Cliente)
    HostChanged(String),
    UserChanged(String),
    PassChanged(String),
    PortChanged(String),
    ConnectPressed,
    DisconnectPressed,
    
    // Atualizações de Input (Servidor)
    ServerPortChanged(String),
    ServerUserChanged(String),
    ServerPassChanged(String),
    StartServerPressed,
    StopServerPressed,
    
    // Respostas Assíncronas
    LogAdded(Tab, String),
    ConnectionStatusChanged(bool),
}

impl Application for FuleZilla {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            FuleZilla {
                active_tab: Tab::Client,
                client_host: String::from("ftp.dominio.com"),
                client_user: String::from("anonymous"),
                client_pass: String::new(),
                client_port: String::from("21"),
                client_connected: false,
                client_log: String::new(),
                
                server_port: String::from("21"),
                server_user: String::from("ads"),
                server_pass: String::from("123"),
                server_running: false,
                server_log: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("FuleZilla v2 - Rust Edition")
    }

    // O Update é o único local onde o estado é mutado
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TabSelected(tab) => {
                self.active_tab = tab;
                Command::none()
            }
            Message::HostChanged(val) => { self.client_host = val; Command::none() }
            Message::UserChanged(val) => { self.client_user = val; Command::none() }
            Message::PassChanged(val) => { self.client_pass = val; Command::none() }
            Message::PortChanged(val) => { self.client_port = val; Command::none() }
            
            Message::ConnectPressed => {
                self.client_connected = true;
                // Aqui você retorna um Command::perform que executa o suppaftp de forma assíncrona
                Command::none() 
            }
            Message::DisconnectPressed => {
                self.client_connected = false;
                Command::none()
            }
            Message::ServerPortChanged(val) => { self.server_port = val; Command::none() }
            Message::ServerUserChanged(val) => { self.server_user = val; Command::none() }
            Message::ServerPassChanged(val) => { self.server_pass = val; Command::none() }
            
            Message::StartServerPressed => {
                self.server_running = true;
                // Aqui você inicia o libunftp via Command e envia mensagens de log de volta
                Command::none()
            }
            Message::StopServerPressed => {
                self.server_running = false;
                Command::none()
            }
            Message::LogAdded(tab, msg) => {
                match tab {
                    Tab::Client => self.client_log.push_str(&format!("{}\n", msg)),
                    Tab::Server => self.server_log.push_str(&format!("{}\n", msg)),
                }
                Command::none()
            }
            Message::ConnectionStatusChanged(status) => {
                self.client_connected = status;
                Command::none()
            }
        }
    }

    // O View reconstrói a interface com base no estado atual a cada frame necessário
    fn view(&self) -> Element<Message> {
        let tabs = row![
            button("🌐 Cliente").on_press(Message::TabSelected(Tab::Client)),
            button("🖥️ Servidor").on_press(Message::TabSelected(Tab::Server)),
        ].spacing(10);

        let content: Element<Message> = match self.active_tab {
            Tab::Client => self.view_client(),
            Tab::Server => self.view_server(),
        };

        let layout = column![tabs, content]
            .spacing(20)
            .padding(20);

        Container::new(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl FuleZilla {
    fn view_client(&self) -> Element<Message> {
        let host_input = text_input("Host", &self.client_host).on_input(Message::HostChanged);
        let user_input = text_input("Usuário", &self.client_user).on_input(Message::UserChanged);
        let pass_input = text_input("Senha", &self.client_pass).on_input(Message::PassChanged).secure(true);
        let port_input = text_input("Porta", &self.client_port).on_input(Message::PortChanged);

        let mut connect_btn = button("Conectar");
        let mut disconnect_btn = button("Desconectar");

        if !self.client_connected {
            connect_btn = connect_btn.on_press(Message::ConnectPressed);
        } else {
            disconnect_btn = disconnect_btn.on_press(Message::DisconnectPressed);
        }

        let connection_panel = column![
            row![text("Host:"), host_input, text("Porta:"), port_input].spacing(10),
            row![text("Usuário:"), user_input, text("Senha:"), pass_input].spacing(10),
            row![connect_btn, disconnect_btn].spacing(10)
        ].spacing(10);

        let log_panel = scrollable(text(&self.client_log)).height(Length::Fixed(150.0));

        column![connection_panel, log_panel].spacing(20).into()
    }

    fn view_server(&self) -> Element<Message> {
        let port_input = text_input("Porta", &self.server_port).on_input(Message::ServerPortChanged);
        let user_input = text_input("Usuário", &self.server_user).on_input(Message::ServerUserChanged);
        let pass_input = text_input("Senha", &self.server_pass).on_input(Message::ServerPassChanged).secure(true);

        let mut start_btn = button("▶️ Iniciar Servidor");
        let mut stop_btn = button("⏹️ Parar Servidor");

        if !self.server_running {
            start_btn = start_btn.on_press(Message::StartServerPressed);
        } else {
            stop_btn = stop_btn.on_press(Message::StopServerPressed);
        }

        let server_panel = column![
            row![text("Porta:"), port_input].spacing(10),
            row![text("Usuário:"), user_input].spacing(10),
            row![text("Senha:"), pass_input].spacing(10),
            row![start_btn, stop_btn].spacing(10)
        ].spacing(10);

        let log_panel = scrollable(text(&self.server_log)).height(Length::Fill);

        column![server_panel, log_panel].spacing(20).into()
    }
}