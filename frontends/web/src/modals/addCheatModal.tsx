import { v4 as uuidv4 } from "uuid";

import { Cheat, CheatType } from "./cheatsModal";

import { CssGrid, GapSize } from "../components/cssGrid";
import { FieldInput } from "../components/form/fieldInput";
import { FieldSelect } from "../components/form/fieldSelect";
import {
    composeValidators,
    maxLength,
    required,
} from "../components/form/validators";
import FormModal from "../components/formModal";
import {
    validateGamesharkCode,
    validateGamegenieCode,
    unregisterCheat,
    registerGamesharkCheat,
    registerGamegenieCheat,
} from "../core/retroboyCore";
import { useSettingsStore } from "../hooks/useSettingsStore";

const cheatTypeOptions = [
    { display: "GameShark", value: "gameshark" },
    { display: "GameGenie", value: "gamegenie" },
];

const AddCheatModal = ({
    gameKey,
    cheat,
    onClose,
}: AddCheatModalProps): JSX.Element => {
    const { settings, storeSettings } = useSettingsStore();

    const handleSend = (
        closeDialog: () => void,
        values: Record<string, unknown>,
    ): Record<string, string> | undefined => {
        const codeValidationError =
            values.type == CheatType.GameShark
                ? validateGamesharkCode(values.code as string)
                : validateGamegenieCode(values.code as string);

        if (codeValidationError) {
            return {
                code:
                    `The provided code is invalid. ` +
                    `Please double check if the code is a valid ${values.type} code.`,
            };
        }

        const cheatToSubmit = {
            id: values.id || uuidv4(),
            name: values.name,
            code: values.code,
            type: values.type,
            registered: values.registered || false,
        } as Cheat;

        if (
            cheat &&
            cheat.id === cheatToSubmit.id &&
            cheat.code !== cheatToSubmit.code &&
            cheatToSubmit.registered
        ) {
            unregisterCheat(cheatToSubmit.id);
            if (cheatToSubmit.type == CheatType.GameShark) {
                registerGamesharkCheat(cheatToSubmit.id, cheatToSubmit.code);
            } else {
                registerGamegenieCheat(cheatToSubmit.id, cheatToSubmit.code);
            }
        }

        storeSettings({
            ...settings,
            cheats: {
                ...settings.cheats,
                [gameKey]: {
                    ...(settings.cheats ? settings.cheats[gameKey] : {}),
                    [cheatToSubmit.id]: cheatToSubmit,
                },
            },
        });

        closeDialog();

        return undefined;
    };

    return (
        <FormModal
            title={cheat ? "Edit Cheat" : "Add Cheat"}
            onClose={onClose}
            initialValues={{
                id: cheat?.id || "",
                name: cheat?.name || "",
                code: cheat?.code || "",
                type: cheat?.type || "",
                registered: cheat?.registered,
            }}
            submitButtonText="Save"
            cancelButtonText="Close"
            onCancel={onClose}
            onSubmit={({ values }) => handleSend(onClose, values)}
        >
            <CssGrid gap={GapSize.medium}>
                <FieldInput
                    label="Name"
                    name="name"
                    validate={composeValidators(required, maxLength(32))}
                />
                <FieldSelect
                    label="Type"
                    name="type"
                    options={cheatTypeOptions}
                    validate={required}
                />
                <FieldInput label="Code" name="code" validate={required} />
            </CssGrid>
        </FormModal>
    );
};

interface AddCheatModalProps {
    readonly gameKey: string;
    readonly cheat: Cheat | null;
    readonly onClose: () => void;
}

export default AddCheatModal;
