<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add Organization</name>
   <tag></tag>
   <elementGuidId>6855f739-5ed3-4858-aa4b-200d6b82f887</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;sdfhggui\&quot;,\n    \&quot;address1\&quot;: \&quot;\&quot;,\n    \&quot;address2\&quot;: \&quot;\&quot;,\n    \&quot;city\&quot;: \&quot;kkkkk\&quot;,\n    \&quot;countryStateId\&quot;: 4,\n    \&quot;FeatureArray\&quot;: [\n        null\n    ],\n    \&quot;postalCode\&quot;: \&quot;4454\&quot;,\n    \&quot;primaryContactFirstName\&quot;: \&quot;rtyu\&quot;,\n    \&quot;primaryContactLastName\&quot;: \&quot;ghv\&quot;,\n    \&quot;primaryContactEmail\&quot;: \&quot;\&quot;,\n    \&quot;primaryContactPhone\&quot;: \&quot;45893145\&quot;,\n    \&quot;secondaryContactFirstName\&quot;: \&quot;tydcmh\&quot;,\n    \&quot;secondaryContactLastName\&quot;: \&quot; dyjjmv\&quot;,\n    \&quot;secondaryContactEmail\&quot;: \&quot;rawal@hsgs.ocn\&quot;,\n    \&quot;secondaryContactPhone\&quot;: \&quot;5896412305\&quot;,\n    \&quot;billingContactFirstName\&quot;: \&quot;chgm\&quot;,\n    \&quot;billingContactLastName\&quot;: \&quot;jgv\&quot;,\n    \&quot;billingContactEmail\&quot;: \&quot;\&quot;,\n    \&quot;billingContactPhone\&quot;: \&quot;\&quot;,\n    \&quot;userLicenses\&quot;: \&quot;\&quot;,\n    \&quot;fileLicenses\&quot;: \&quot;\&quot;,\n    \&quot;storageLicenses\&quot;: \&quot;5\&quot;,\n    \&quot;features\&quot;: []\n}&quot;,
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
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJodHRwOi8vc2NoZW1hcy54bWxzb2FwLm9yZy93cy8yMDA1LzA1L2lkZW50aXR5L2NsYWltcy9uYW1laWRlbnRpZmllciI6IjkiLCJPcmdJZCI6IjEiLCJjblN0ciI6Im90bEZoWjFhRWFsbDdhNk9Bd1ozaU5iZkVwRXBwUzEwYTN4ZWh2Wm5RZzduNzMrZXhmUUxSVFZ4ZEJOUUluOEJ6a1FVa3pibnBPU3EybmVENDJEWHMwVnFmVVVlazdoWTBpRkNXazQ4eldvPSIsImh0dHA6Ly9zY2hlbWFzLm1pY3Jvc29mdC5jb20vd3MvMjAwOC8wNi9pZGVudGl0eS9jbGFpbXMvcm9sZSI6IkRCQSIsImV4cCI6MTU1NDQ3NDI5OSwiaXNzIjoiaHR0cDovL2xvY2FsaG9zdDo1MDAwIiwiYXVkIjoiaHR0cDovL2xvY2FsaG9zdDo1MDAwIn0.B1yauDCfWi-zHVlJ4Z95-hL0t2sGgeqNGBd_WvtHsl8</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://dev-dweb.cygrp.com:5000//api/organization/AddOrganization</restUrl>
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
