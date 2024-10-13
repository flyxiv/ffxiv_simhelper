export const EQUIPMENT_TABLE_ITEM_MIN_WIDTH_PX = 600;

export const EQUIPMENT_CONTAINER_MIN_WIDTH_PX = (itemsInOneRow: number) =>
    itemsInOneRow === 1 ? `${EQUIPMENT_TABLE_ITEM_MIN_WIDTH_PX + 100}px` : `${EQUIPMENT_TABLE_ITEM_MIN_WIDTH_PX * 2 + 100}px`


export const MATERIA_MIN_WIDTH_PX = (numberOfMaterias: number) => `${EQUIPMENT_TABLE_ITEM_MIN_WIDTH_PX / numberOfMaterias}px`; 
