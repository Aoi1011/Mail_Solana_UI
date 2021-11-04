import { pageTypes } from "../action_types";

export function setActivePage(page: any) {
  return async (dispatch: any) => {
    dispatch({
      type: pageTypes.SET_SELECTED_PAGE,
      page,
    });
  };
}
