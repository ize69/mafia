import React, { ReactElement, useContext, useEffect, useMemo, useState } from "react";
import GAME_MANAGER from "../../index";
import LobbyPlayerList from "./LobbyPlayerList";
import "./lobbyMenu.css";
import translate from "../../game/lang";
import { StateListener } from "../../game/gameManager.d";
import { AnchorContext } from "../Anchor";
import { RoomLinkButton } from "../GlobalMenu";
import { RoleList, getAllRoles } from "../../game/roleListState.d";
import LoadingScreen from "../LoadingScreen";
import StartMenu from "../main/StartMenu";
import { GameModeContext } from "../../components/gameModeSettings/GameModesEditor";
import PhaseTimesSelector from "../../components/gameModeSettings/PhaseTimeSelector";
import { OutlineListSelector } from "../../components/gameModeSettings/OutlineSelector";
import EnabledRoleSelector from "../../components/gameModeSettings/EnabledRoleSelector";
import Icon from "../../components/Icon";
import { GameModeSelector } from "../../components/gameModeSettings/GameModeSelector";
import LobbyChatMenu from "./LobbyChatMenu";
import { useLobbyState } from "../../components/useHooks";
import { Button } from "../../components/Button";

export default function LobbyMenu(): ReactElement {
    const isSpectator = useLobbyState(
        lobbyState => lobbyState.players.get(lobbyState.myId!)?.clientType.type === "spectator",
        ["playersHost", "lobbyClients"]
    )!;
    const isHost = useLobbyState(
        lobbyState => lobbyState.players.get(lobbyState.myId!)?.host,
        ["playersHost", "lobbyClients"]
    )!;
    const { mobile } = useContext(AnchorContext)!;

    const [advancedView, setAdvancedView] = useState<boolean>(isHost || mobile);

    useEffect(() => {
        setAdvancedView(isHost || mobile)
    }, [mobile, isHost])

    return <div className="lm">
        <LobbyMenuHeader advancedView={advancedView} setAdvancedView={setAdvancedView}/>
        {advancedView 
            ? <main>
                <div>
                    <LobbyPlayerList />
                    <LobbyChatMenu spectator={isSpectator}/>
                </div>
                <div>
                    <LobbyMenuSettings isHost={isHost}/>
                </div>
            </main>
            : <main>
                <div>
                    <LobbyPlayerList />
                </div>
                <div>
                    <LobbyChatMenu spectator={isSpectator}/>
                </div>
            </main>
        }
    </div>
}

function LobbyMenuSettings(props: Readonly<{
    isHost: boolean,
}>): JSX.Element {
    const roleList = useLobbyState(
        lobbyState => lobbyState.roleList,
        ["roleList", "roleOutline"]
    )!;
    const enabledRoles = useLobbyState(
        lobbyState => lobbyState.enabledRoles,
        ["enabledRoles"]
    )!;
    const phaseTimes = useLobbyState(
        lobbyState => lobbyState.phaseTimes,
        ["phaseTimes"]
    )!;

    const { mobile, setContent: setAnchorContent } = useContext(AnchorContext)!;

    useEffect(() => {
        const listener: StateListener = (type) => {
            if(type === "rejectJoin"){
                // Kicked, probably
                setAnchorContent(<LoadingScreen type="disconnect"/>);
                GAME_MANAGER.setDisconnectedState();
                setAnchorContent(<StartMenu />);
            }
        }

        GAME_MANAGER.addStateListener(listener);
        return ()=>{GAME_MANAGER.removeStateListener(listener);}
    }, [setAnchorContent]);

    const sendRoleList = (newRoleList: RoleList) => {
        const combinedRoleList = [...roleList];
        newRoleList.forEach((role, index) => {
            combinedRoleList[index] = role
        })
        GAME_MANAGER.sendSetRoleListPacket(combinedRoleList);
    };

    const context = useMemo(() => {
        return {roleList, enabledRoles, phaseTimes};
    }, [enabledRoles, phaseTimes, roleList]);

    return <GameModeContext.Provider value={context}>
        {mobile && <h1>{translate("menu.lobby.settings")}</h1>}
        {props.isHost && <GameModeSelector 
            canModifySavedGameModes={false}
            loadGameMode={gameMode => {
                GAME_MANAGER.sendSetPhaseTimesPacket(gameMode.phaseTimes);
                GAME_MANAGER.sendEnabledRolesPacket(gameMode.enabledRoles);
                GAME_MANAGER.sendSetRoleListPacket(gameMode.roleList);
            }}
        />}
        <PhaseTimesSelector 
            disabled={!props.isHost}
            onChange={pts => GAME_MANAGER.sendSetPhaseTimesPacket(pts)}
        />
        <OutlineListSelector
            disabled={!props.isHost}
            onChangeRolePicker={(value, index) => GAME_MANAGER.sendSetRoleOutlinePacket(index, value)}
            onAddNewOutline={undefined}
            onRemoveOutline={undefined}
            setRoleList={sendRoleList}
        />
        <EnabledRoleSelector
            onEnableRoles={roles => GAME_MANAGER.sendEnabledRolesPacket([...enabledRoles, ...roles])}
            onDisableRoles={roles => GAME_MANAGER.sendEnabledRolesPacket(enabledRoles.filter(role => !roles.includes(role)))}
            onIncludeAll={() => GAME_MANAGER.sendEnabledRolesPacket(getAllRoles())}
            disabled={!props.isHost}
        />
    </GameModeContext.Provider>
}

// There's probably a better way to do this that doesn't need the mobile check.
function LobbyMenuHeader(props: Readonly<{
    advancedView: boolean,
    setAdvancedView: (advancedView: boolean) => void
}>): JSX.Element {
    const [lobbyName, setLobbyName] = useState<string>(GAME_MANAGER.state.stateType === "lobby" ? GAME_MANAGER.state.lobbyName : "Mafia Lobby");
    const host = useLobbyState(
        lobbyState => lobbyState.players.get(lobbyState.myId!)?.host,
        ["lobbyClients", "yourId", "playersHost"]
    )!;
    const { mobile, setContent: setAnchorContent } = useContext(AnchorContext)!;

    useEffect(() => {
        const listener: StateListener = (type) => {
            if (type === "lobbyName" && GAME_MANAGER.state.stateType === "lobby") {
                setLobbyName(GAME_MANAGER.state.lobbyName);
            }
        };

        if(GAME_MANAGER.state.stateType === "lobby")
            setLobbyName(GAME_MANAGER.state.lobbyName);

        GAME_MANAGER.addStateListener(listener)
        return ()=>{GAME_MANAGER.removeStateListener(listener);}
    }, [setLobbyName]);

    return <header>
        <div>
            <button disabled={!host} className="start" onClick={async ()=>{
                setAnchorContent(<LoadingScreen type="default"/>);
                if (!await GAME_MANAGER.sendStartGamePacket()) {
                    setAnchorContent(<LobbyMenu/>)
                }
            }}>
                <Icon>play_arrow</Icon>{translate("menu.lobby.button.start")}
            </button>
            <RoomLinkButton/>
            {mobile || host || <Button
                onClick={() => props.setAdvancedView(!props.advancedView)}
            >
                <Icon>settings</Icon>{translate(`menu.lobby.button.advanced.${props.advancedView}`)}
            </Button>}
        </div>
        { host ? 
            <input 
                type="text" 
                value={lobbyName}
                onInput={e => {
                    setLobbyName((e.target as HTMLInputElement).value);
                }}
                onKeyUp={(e)=>{
                    if(e.key !== 'Enter') return;
                    
                    const newLobbyName = (e.target as HTMLInputElement).value;
                    setLobbyName(newLobbyName);
                    GAME_MANAGER.sendSetLobbyNamePacket(newLobbyName);
                    
                }}
                onBlur={e => {
                    const newLobbyName = (e.target as HTMLInputElement).value;
                    setLobbyName(newLobbyName);
                    GAME_MANAGER.sendSetLobbyNamePacket(newLobbyName);
                }}
            /> : 
            <h1>{lobbyName}</h1>
        }
        
    </header>
}

