import { pageTypes } from "../action_types";

export function page(state = {}, action: any) {
  switch (action.type) {
    case pageTypes.SET_SELECTED_PAGE:
      return {
        selectedPage: action.page,
      };
    default:
      return state;
  }
}
