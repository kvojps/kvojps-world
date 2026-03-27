use bevy::prelude::*;

#[derive(Component)]
pub(super) struct MainMenuUiRoot;

#[derive(Component)]
pub(super) struct MainMenuPanel;

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
            Self::Options => "Opções",
            Self::Quit => "Sair",
        }
    }
}

#[derive(Component, Clone, Copy)]
pub(super) struct MenuItemActionButton(pub MenuItem);

#[derive(Component, Clone, Copy)]
pub(super) struct MenuItemLabel(pub MenuItem);
