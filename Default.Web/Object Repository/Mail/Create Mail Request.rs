<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Mail Request</name>
   <tag></tag>
   <elementGuidId>7eaa9d01-be80-4d23-8235-2ddf2fe67da2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;description\&quot;: \&quot;tesst\&quot;,\n    \&quot;ActionDate\&quot;: \&quot;2019-03-29T18:30:00.000Z\&quot;,\n    \&quot;organizationId\&quot;: 1,\n    \&quot;senderInfo\&quot;: {\n        \&quot;FullName\&quot;: \&quot;TestPerson\&quot;,\n        \&quot;City\&quot;: \&quot;Addison\&quot;,\n        \&quot;Address1\&quot;: \&quot;15000 Surveyor Boulevard\&quot;,\n        \&quot;Address2\&quot;: \&quot;Suite 100\&quot;,\n        \&quot;CountryStateID\&quot;: 1,\n        \&quot;PostalCode\&quot;: \&quot;1234\&quot;\n    },\n    \&quot;addDocumentRequest\&quot;: [\n        {\n            \&quot;recipients\&quot;: [\n                {\n                    \&quot;mailType\&quot;: \&quot;0\&quot;,\n                    \&quot;mailService\&quot;: 2,\n                    \&quot;mailEnclosures\&quot;: 2,\n                    \&quot;receiverinfo\&quot;: {\n                        \&quot;FullName\&quot;: \&quot;TestPerson\&quot;,\n                        \&quot;address1\&quot;: \&quot;TestPerson\&quot;,\n                        \&quot;address2\&quot;: \&quot;TestPerson2\&quot;,\n                        \&quot;PostalCode\&quot;: \&quot;11111\&quot;,\n                        \&quot;city\&quot;: \&quot;New Delhi\&quot;,\n                        \&quot;CountryStateID\&quot;: 1\n                    }\n                }\n            ],\n            \&quot;fileName\&quot;: \&quot;csharp.pdf\&quot;,\n            \&quot;location\&quot;: \&quot;csharp.pdf\&quot;,\n            \&quot;printingInfo\&quot;: {\n                \&quot;isBannerIncluded\&quot;: false,\n                \&quot;isSinglePlex\&quot;: 1,\n                \&quot;isDocumentRetained\&quot;: 0\n            }\n        }\n    ]\n}&quot;,
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
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJodHRwOi8vc2NoZW1hcy54bWxzb2FwLm9yZy93cy8yMDA1LzA1L2lkZW50aXR5L2NsYWltcy9uYW1laWRlbnRpZmllciI6IjkiLCJPcmdJZCI6IjEiLCJjblN0ciI6Im90bEZoWjFhRWFsbDdhNk9Bd1ozaU5iZkVwRXBwUzEwYTN4ZWh2Wm5RZzduNzMrZXhmUUxSVFZ4ZEJOUUluOEJ6a1FVa3pibnBPU3EybmVENDJEWHMwVnFmVVVlazdoWTBpRkNXazQ4eldvPSIsImh0dHA6Ly9zY2hlbWFzLm1pY3Jvc29mdC5jb20vd3MvMjAwOC8wNi9pZGVudGl0eS9jbGFpbXMvcm9sZSI6IkRCQSIsImV4cCI6MTU1NDM2MDIwNywiaXNzIjoiaHR0cDovL2xvY2FsaG9zdDo1MDAwIiwiYXVkIjoiaHR0cDovL2xvY2FsaG9zdDo1MDAwIn0.evcsapHtDRrjsL9cAmPL2PRIPuQQ9W993Rp6JU7LrhQ</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://dev-dweb.cygrp.com:5000//api/mail/createmailrequest</restUrl>
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
