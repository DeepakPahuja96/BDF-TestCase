import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
// Below libraries were imported manually
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import com.kms.katalon.core.testobject.TestObjectProperty as TestObjectProperty
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import groovy.json.JsonSlurper as JsonSlurper

RequestObject signInRequest = findTestObject('Auth/Sign In') 
//
signInResponse = WS.sendRequest(signInRequest) 
//
JsonSlurper jsonSlurper = new JsonSlurper()
//
Map signInResponseBody = jsonSlurper.parseText(signInResponse.getResponseBodyContent())
//
TestObjectProperty authHeader = new TestObjectProperty('Authorization', ConditionType.EQUALS,  
															'Bearer ' + signInResponseBody.data.token)  
//
GlobalVariable.Authorization = 'Bearer ' + signInResponseBody.data.token  

// -------------------------    COMMENT above lines if test suite collection is running

// -------------------------    UNCOMMENT below 2 lines if test suite collection is running


//TestObjectProperty authHeader = new TestObjectProperty('Authorization', ConditionType.EQUALS,
//														'Bearer ' + GlobalVariable.Authorization)

println(GlobalVariable.Authorization)

RequestObject request = findTestObject('Org Admin/Get Organization Detail By ID')

request.httpHeaderProperties.add(authHeader)

response = WS.sendRequest(request)

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'message', 'success')

WS.verifyElementPropertyValue(response, 'isSuccess', true)

