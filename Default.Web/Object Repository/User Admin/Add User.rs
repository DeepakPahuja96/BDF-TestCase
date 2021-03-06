<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add User</name>
   <tag></tag>
   <elementGuidId>0b347157-962f-4986-a172-de9701019d58</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;UserName\&quot;: \&quot;dddw@c.com\&quot;,\n    \&quot;textTempPassword\&quot;: \&quot;cmkd\&quot;,\n    \&quot;FirstName\&quot;: \&quot;sed\&quot;,\n    \&quot;MiddleName\&quot;: \&quot;y\&quot;,\n    \&quot;LastName\&quot;: \&quot;ftiut\&quot;,\n    \&quot;Address1\&quot;: \&quot;vgh\&quot;,\n    \&quot;Address2\&quot;: \&quot;\&quot;,\n    \&quot;PostalCode\&quot;: \&quot;dad\&quot;,\n    \&quot;EmailAddress\&quot;: \&quot;vkn@gs.coj\&quot;,\n    \&quot;PrimaryPhone\&quot;: \&quot;23242\&quot;,\n    \&quot;StateID\&quot;: 111111,\n    \&quot;CountryStateID\&quot;: [\n      \n    ],\n    \&quot;ClientGroupIDList\&quot;: [\n       \n    ],\n    \&quot;City\&quot;: \&quot;esase.fca\&quot;,\n    \&quot;Features\&quot;: [],\n    \&quot;IsUserActive\&quot;: true,\n    \&quot;RoleID\&quot;: [\n        2,\n        4\n    ],\n    \&quot;UserPassword\&quot;: \&quot;mfkcw\&quot;,\n    \&quot;ContactTypeID\&quot;: 5,\n    \&quot;UserTypeID\&quot;: 1,\n    \&quot;OrganizationID\&quot;: \&quot;\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJodHRwOi8vc2NoZW1hcy54bWxzb2FwLm9yZy93cy8yMDA1LzA1L2lkZW50aXR5L2NsYWltcy9uYW1laWRlbnRpZmllciI6IjkiLCJPcmdJZCI6IjEiLCJjblN0ciI6Im90bEZoWjFhRWFsbDdhNk9Bd1ozaU5iZkVwRXBwUzEwYTN4ZWh2Wm5RZzduNzMrZXhmUUxSVFZ4ZEJOUUluOEJ6a1FVa3pibnBPU3EybmVENDJEWHMwVnFmVVVlazdoWTBpRkNXazQ4eldvPSIsImh0dHA6Ly9zY2hlbWFzLm1pY3Jvc29mdC5jb20vd3MvMjAwOC8wNi9pZGVudGl0eS9jbGFpbXMvcm9sZSI6IkRCQSIsImV4cCI6MTU1NDcwNzgxMSwiaXNzIjoiaHR0cDovL2xvY2FsaG9zdDo1MDAwIiwiYXVkIjoiaHR0cDovL2xvY2FsaG9zdDo1MDAwIn0.R1UxeI-qmAMzM0kZBI4xMKZQyJYatQ79jwaiNCKoWJs</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://dev-dweb.cygrp.com:5000//api/user/adduser</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
