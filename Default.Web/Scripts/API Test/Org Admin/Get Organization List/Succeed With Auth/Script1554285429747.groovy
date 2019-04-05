import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.testobject.ConditionType as ConditionType
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import com.kms.katalon.core.testobject.TestObjectProperty as TestObjectProperty
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import groovy.json.JsonSlurper as JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject signInRequest = findTestObject('Object Repository/Auth/Sign In')

//println(signInRequest)

signInResponse = WS.sendRequest(signInRequest)

RequestObject request = findTestObject('Object Repository/Org Admin/Get Organization List')

JsonSlurper jsonSlurper = new JsonSlurper()

Map signInResponseBody = jsonSlurper.parseText(signInResponse.getResponseBodyContent())
//
//CustomKeywords.'GlobalVariableHelper.addGlobalVariable'('Authorization', 'Bearer ' + signInResponseBody.data.token)
//GlobalVariable.Authorization = 'Bearer ' + signInResponseBody.data.token

//println(GlobalVariable.Authorization)

TestObjectProperty authHeader = new TestObjectProperty('Authorization', ConditionType.EQUALS, 'Bearer ' + signInResponseBody.data.token)

request.httpHeaderProperties.add(authHeader)

GlobalVariable.Authorization = 'Bearer ' + signInResponseBody.data.token

response = WS.sendRequest(request)

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'isSuccess', true)


