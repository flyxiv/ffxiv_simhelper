import { EquipmentInput } from "src/types/EquipmentInput";
import { defaultSingleEquipmentInput } from "./DefaultSingleEquipmentInput";

export function defaultDoubleEquipmentInput(): EquipmentInput {
    let singleEquipmentInput = defaultSingleEquipmentInput();
    let singleEquipmentInput2 = defaultSingleEquipmentInput();

    return {
        equipmentDatas: [singleEquipmentInput.equipmentDatas[0], singleEquipmentInput2.equipmentDatas[0]]
    }
}