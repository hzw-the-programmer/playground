package handlers

import (
	"net/http"
	"strconv"

	"github.com/gin-contrib/sessions"
	"github.com/gin-gonic/gin"

	"g1.hzw/rpc"
)

func InputPhoneNumber(c *gin.Context) {
	session := sessions.Default(c)

	countryCode := session.Get("countryCode")
	phoneNumber := session.Get("phoneNumber")
	flashes := session.Flashes()

	session.Save()

	c.HTML(http.StatusOK, "input_phone_number.html", gin.H{
		"countryCodes": CountryCodes,
		"countryCode":  countryCode,
		"phoneNumber":  phoneNumber,
		"flashes":      flashes,
	})
}

func SendVerifyCode(c *gin.Context) {
	session := sessions.Default(c)

	flash := ""

	countryCodeStr := c.PostForm("countryCode")
	phoneNumberStr := c.PostForm("phoneNumber")

	session.Set("countryCode", countryCodeStr)
	session.Set("phoneNumber", phoneNumberStr)

	phoneNumber, err := strconv.ParseInt(countryCodeStr+phoneNumberStr, 10, 64)
	if err != nil {
		flash = "Phone number is wrong"
		goto redirect
	}

	err = rpc.SendVerifyCode(phoneNumber)
	if err != nil {
		flash = err.Error()
		goto redirect
	}

	session.Save()
	c.Redirect(http.StatusSeeOther, "/input_verify_code")

	return

redirect:
	session.AddFlash(flash)
	session.Save()
	c.Redirect(http.StatusSeeOther, "/input_phone_number")
}

func InputVerifyCode(c *gin.Context) {
	session := sessions.Default(c)

	flash := ""

	countryCodeStr := session.Get("countryCode").(string)
	phoneNumberStr := session.Get("phoneNumber").(string)

	phoneNumber, err := strconv.ParseInt(countryCodeStr+phoneNumberStr, 10, 64)
	if err != nil {
		flash = "Phone number is wrong"
		goto redirect
	}

	c.HTML(http.StatusOK, "input_verify_code.html", gin.H{
		"phoneNumber": phoneNumber,
	})

	return

redirect:
	session.AddFlash(flash)
	session.Save()
	c.Redirect(http.StatusSeeOther, "/input_phone_number")
}

var CountryCodes = []struct {
	Code string
	Name string
}{
	{"86", "China"},
	{"213", "Algeria"},
	{"244", "Angola"},
	{"229", "Benin"},
	{"245", "Bissau Guinea"},
	{"267", "Botswana"},
	{"226", "Burkina Faso"},
	{"257", "Burundi"},
	{"237", "Cameroon"},
	{"236", "Central Africa Republic"},
	{"235", "Chad"},
	{"243", "Congo Dem. Rep."},
	{"242", "Congo Republic"},
	{"253", "Djibouti"},
	{"971", "Dubai"},
	{"20", "Egypt"},
	{"240", "Equatorial Guinea"},
	{"251", "Ethiopia"},
	{"241", "Gabonese"},
	{"220", "Gambia"},
	{"233", "Ghana"},
	{"224", "Guinea"},
	{"98", "Iran"},
	{"225", "Ivory Coast"},
	{"254", "Kenya"},
	{"231", "Liberia"},
	{"218", "Libya"},
	{"261", "Madagascar"},
	{"265", "Malawi"},
	{"223", "Mali"},
	{"222", "Mauritania"},
	{"230", "Mauritius"},
	{"262", "Mayotte"},
	{"212", "Morocco"},
	{"258", "Mozambique"},
	{"264", "Namibia"},
	{"227", "Niger"},
	{"234", "Nigeria"},
	{"262", "Reunion"},
	{"250", "Rwanda"},
	{"221", "Senegal"},
	{"232", "Sierra Leone"},
	{"677", "Solomon"},
	{"252", "Somalia"},
	{"27", "South Africa"},
	{"211", "South Sudan"},
	{"249", "Sudan"},
	{"268", "Swaziland"},
	{"255", "Tanzania"},
	{"228", "Togo"},
	{"216", "Tunisia"},
	{"256", "Uganda"},
	{"260", "Zambia"},
	{"263", "Zimbabwe"},
}
