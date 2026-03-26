use bevy::prelude::*;

pub(super) const MENU_STACK_BREAKPOINT: f32 = 760.0;

#[derive(Component)]
pub(super) struct MainMenuUiRoot;

#[derive(Component)]
pub(super) struct MainMenuPanel;

#[derive(Component)]
pub(super) struct MainMenuDescriptionText;

#[derive(Component)]
pub(super) struct MainMenuHintText;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub(super) enum MenuItem {
    NewGame,
    LoadGame,
    Options,
    Quit,
}

impl MenuItem {
    pub(super) const ALL: [Self; 4] = [Self::NewGame, Self::LoadGame, Self::Options, Self::Quit];

    pub(super) fn index(self) -> usize {
        match self {
            Self::NewGame => 0,
            Self::LoadGame => 1,
            Self::Options => 2,
            Self::Quit => 3,
        }
    }

    pub(super) fn label(self) -> &'static str {
        match self {
            Self::NewGame => "Novo Jogo",
            Self::LoadGame => "Carregar Jornada",
            Self::Options => "Opcoes",
            Self::Quit => "Sair",
        }
    }

    pub(super) fn description(self) -> &'static str {
        match self {
            Self::NewGame => "Inicie uma nova aventura no reino de Kvojps.",
            Self::LoadGame => "Retome a jornada a partir do ultimo acampamento.",
            Self::Options => "Ajuste audio, controles e preferencias visuais.",
            Self::Quit => "Fechar o jogo e voltar ao desktop.",
        }
    }
}

#[derive(Component, Clone, Copy)]
pub(super) struct MainMenuActionButton(pub MenuItem);

#[derive(Component, Clone, Copy)]
pub(super) struct MainMenuOptionLabel(pub MenuItem);
