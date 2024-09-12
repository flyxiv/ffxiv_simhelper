import { MenuItem } from "@mui/material";
import { PartnerItem } from "./PartnerItem";

export function PartnerMenuItem(id: number, partnerMemberId: number, jobAbbrev: string) {
  return <MenuItem key={`${id}_${partnerMemberId}_${jobAbbrev}_menuitem`} value={partnerMemberId}>{PartnerItem(partnerMemberId, jobAbbrev)}</MenuItem>;
}
