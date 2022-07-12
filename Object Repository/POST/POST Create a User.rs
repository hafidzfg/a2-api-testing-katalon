<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Create a User</name>
   <tag></tag>
   <elementGuidId>157eea1c-bac5-4f5b-98e2-815c7e71d2c2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;${name}\&quot;,\n    \&quot;username\&quot;: \&quot;${username}\&quot;,\n    \&quot;email\&quot;: \&quot;${email}\&quot;,\n    \&quot;address\&quot;: {\n      \&quot;street\&quot;: \&quot;${street}\&quot;,\n      \&quot;suite\&quot;: \&quot;${suite}\&quot;,\n      \&quot;city\&quot;: \&quot;${city}\&quot;,\n      \&quot;zipcode\&quot;: \&quot;${zipcode}\&quot;,\n      \&quot;geo\&quot;: {\n        \&quot;lat\&quot;: \&quot;${lat}\&quot;,\n        \&quot;lng\&quot;: \&quot;${lng}\&quot;\n      }\n    }\n}&quot;,
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
      <webElementGuid>fa5b6130-0890-4652-a731-047401d9ec11</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://jsonplaceholder.typicode.com/users</restUrl>
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


WS.verifyElementPropertyValue(response, 'name', 'Leanne Graham')
WS.verifyElementPropertyValue(response, 'username', 'Bret')
WS.verifyElementPropertyValue(response, 'email', 'Sincere@april.biz')
WS.verifyElementPropertyValue(response, 'address.street', 'Kulas Light')
WS.verifyElementPropertyValue(response, 'address.suite', 'Apt. 556')
WS.verifyElementPropertyValue(response, 'address.city', 'Gwenborough')
WS.verifyElementPropertyValue(response, 'address.zipcode', '92998-3874')
WS.verifyElementPropertyValue(response, 'address.geo.lat', '-37.3159')
WS.verifyElementPropertyValue(response, 'address.geo.lng', '81.1496')
WS.verifyElementPropertyValue(response, 'id', 11)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
