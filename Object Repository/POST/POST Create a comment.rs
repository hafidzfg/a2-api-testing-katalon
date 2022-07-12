<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Create a comment</name>
   <tag></tag>
   <elementGuidId>11e4617a-5c75-48b7-88eb-77200e50b9b9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;${name}\&quot;,\n    \&quot;email\&quot;: \&quot;${email}\&quot;,\n    \&quot;body\&quot;: \&quot;${body}\&quot;\n}&quot;,
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
      <webElementGuid>f89c4ec0-fff5-4bea-8605-12f2a86277b7</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://jsonplaceholder.typicode.com/comments</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)


WS.verifyElementPropertyValue(response, 'name', 'supreme general')
WS.verifyElementPropertyValue(response, 'email', 'sg.1998@outlook.com')
WS.verifyElementPropertyValue(response, 'body', '1. Mengambil segala tindakan yang dianggap perlu untuk terjaminnya keamanan dan ketenangan serta kestabilan jalannya pemerintahan dan jalannya Revolusi, serta menjamin keselamatan pribadi dan kewibawaan Pimpinan Presiden/Panglima Tertinggi/Pemimpin Besar Revolusi/Mandataris MPRS, demi untuk keutuhan Bangsa dan Negara Republik Indonesia, dan melaksanakan dengan pasti segala ajaran Pemimpin Besar Revolusi.')
WS.verifyElementPropertyValue(response, 'id', 501)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
