<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET All Todos</name>
   <tag></tag>
   <elementGuidId>f17bfa79-2af0-453b-af18-13843b7eed4d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://jsonplaceholder.typicode.com/todos</restUrl>
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

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, '[0].userId', 1)
WS.verifyElementPropertyValue(response, '[0].id', 1)
WS.verifyElementPropertyValue(response, '[0].title', 'delectus aut autem')
WS.verifyElementPropertyValue(response, '[0].completed', false)

WS.verifyElementPropertyValue(response, '[1].userId', 1)
WS.verifyElementPropertyValue(response, '[1].id', 2)
WS.verifyElementPropertyValue(response, '[1].title', 'quis ut nam facilis et officia qui')
WS.verifyElementPropertyValue(response, '[1].completed', false)

WS.verifyElementPropertyValue(response, '[2].userId', 1)
WS.verifyElementPropertyValue(response, '[2].id', 3)
WS.verifyElementPropertyValue(response, '[2].title', 'fugiat veniam minus')
WS.verifyElementPropertyValue(response, '[2].completed', false)

WS.verifyElementPropertyValue(response, '[3].userId', 1)
WS.verifyElementPropertyValue(response, '[3].id', 4)
WS.verifyElementPropertyValue(response, '[3].title', 'et porro tempora')
WS.verifyElementPropertyValue(response, '[3].completed', true)

WS.verifyElementPropertyValue(response, '[4].userId', 1)
WS.verifyElementPropertyValue(response, '[4].id', 5)
WS.verifyElementPropertyValue(response, '[4].title', 'laboriosam mollitia et enim quasi adipisci quia provident illum')
WS.verifyElementPropertyValue(response, '[4].completed', false)

WS.verifyElementPropertyValue(response, '[5].userId', 1)
WS.verifyElementPropertyValue(response, '[5].id', 6)
WS.verifyElementPropertyValue(response, '[5].title', 'qui ullam ratione quibusdam voluptatem quia omnis')
WS.verifyElementPropertyValue(response, '[5].completed', false)

WS.verifyElementPropertyValue(response, '[6].userId', 1)
WS.verifyElementPropertyValue(response, '[6].id', 7)
WS.verifyElementPropertyValue(response, '[6].title', 'illo expedita consequatur quia in')
WS.verifyElementPropertyValue(response, '[6].completed', false)

WS.verifyElementPropertyValue(response, '[7].userId', 1)
WS.verifyElementPropertyValue(response, '[7].id', 8)
WS.verifyElementPropertyValue(response, '[7].title', 'quo adipisci enim quam ut ab')
WS.verifyElementPropertyValue(response, '[7].completed', true)

WS.verifyElementPropertyValue(response, '[8].userId', 1)
WS.verifyElementPropertyValue(response, '[8].id', 9)
WS.verifyElementPropertyValue(response, '[8].title', 'molestiae perspiciatis ipsa')
WS.verifyElementPropertyValue(response, '[8].completed', false)

WS.verifyElementPropertyValue(response, '[9].userId', 1)
WS.verifyElementPropertyValue(response, '[9].id', 10)
WS.verifyElementPropertyValue(response, '[9].title', 'illo est ratione doloremque quia maiores aut')
WS.verifyElementPropertyValue(response, '[9].completed', true)

WS.verifyElementPropertyValue(response, '[10].userId', 1)
WS.verifyElementPropertyValue(response, '[10].id', 11)
WS.verifyElementPropertyValue(response, '[10].title', 'vero rerum temporibus dolor')
WS.verifyElementPropertyValue(response, '[10].completed', true)


</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
