export const SUBMIT_PRICE_FORM = 'SUBMIT_PRICE_FORM'
export const SUBMIT_PRICE_FORM_REQUEST = 'SUBMIT_PRICE_FORM_REQUEST'
export const SUBMIT_PRICE_FORM_SUCCESS = 'SUBMIT_PRICE_FORM_SUCCESS'
export const SUBMIT_PRICE_FORM_FAILURE = 'SUBMIT_PRICE_FORM_FAILURE'

const action = (type, payload = {}) => ({type, ...payload})

export const submitPriceForm = action.bind(null, SUBMIT_PRICE_FORM)
export const submitPriceFormRequest = action.bind(null, SUBMIT_PRICE_FORM_REQUEST)
export const submitPriceFormSuccess = action.bind(null, SUBMIT_PRICE_FORM_SUCCESS)
export const submitPriceFormFailure = action.bind(null, SUBMIT_PRICE_FORM_FAILURE)
