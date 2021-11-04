import { fetchData, send } from "../../services";
import { mailTypes } from "../action_types";

export function getAccountData() {
  return async (dispatch: any, getState: any) => {
    dispatch(request());

    try {
      const accountId = getState().account.accountId;

      const mailAccount = await fetchData(accountId);

      dispatch(
        success({
          inbox: mailAccount.inbox,
          sent: mailAccount.sent,
        })
      );
    } catch (err: any) {
      console.log(err);
      dispatch(failed({ message: err.message }));
    }
  };

  function request() {
    return { type: mailTypes.GET_REQUEST };
  }
  function success(payload: any) {
    return { type: mailTypes.GET_SUCCESS, payload };
  }
  function failed(payload: any) {
    return { type: mailTypes.GET_FAILURE, payload };
  }
}

export function sendMail(mail: any) {
  return async (dispatch: any, getState: any) => {
    dispatch(request());

    try {
      const programId = getState().account.programId;
      const wallet = getState().account.wallet;

      await send(mail, programId, wallet);

      const accountId = getState().account.accountId;
      const mailAccount = await fetchData(accountId);

      dispatch(
        success({
          inbox: mailAccount.inbox,
          sent: mailAccount.sent,
        })
      );
    } catch (error: any) {
      console.log(error);
      dispatch(failed({ message: error.message }));
    }
  };

  function request() {
    return {
      types: mailTypes.SEND_REQUEST,
    };
  }

  function success(payload: any) {
    return {
      types: mailTypes.SEND_SUCCESS,
      payload,
    };
  }

  function failed(payload: any) {
    return {
      type: mailTypes.SEND_FAILURE,
      payload,
    };
  }
}
