<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Token_Fiona_FIF</name>
   <tag></tag>
   <elementGuidId>8336ff24-16ca-49fd-9178-693a4d783e87</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;userId\&quot;: \&quot;${user}\&quot;,\n    \&quot;password\&quot;: \&quot;${pass}\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>e0f13159-420c-4048-b945-57161b863d16</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://fifada-qa-lb01.fifgroup.co.id/backend/user/login</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'fiona'</defaultValue>
      <description></description>
      <id>04ca75db-f4be-4705-9bbf-1de964171885</id>
      <masked>false</masked>
      <name>user</name>
   </variables>
   <variables>
      <defaultValue>'zQe9Rk4qqvzs3Xyz'</defaultValue>
      <description></description>
      <id>4b5a0cdb-f4be-437c-a70d-1d36d599a85b</id>
      <masked>false</masked>
      <name>pass</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

GlobalVariable.token = WS.getElementPropertyValue(response, 'result.accessToken')
System.out.println(GlobalVariable.token)

//buat dapatin result value access token
//WS.verifyElementPropertyValue(response, 'result.accessToken', &quot;eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJ7XCJ1c2VySWRcIjpcImZpb25hXCIsXCJ0eXBlXCI6XCJCQUNLRU5EQURNSU5cIixcImJhY2tlbmRVc2VySWRcIjpcImZpb25hXCJ9IiwianRpIjoiYTc2MGYzNmItMmE3YS00ZmU3LWE2ZmEtNjc1Njc5YTkwOGE0IiwiaWF0IjoxNzA2MDcxMDcxLCJleHAiOjE3MDYwNzQ2NzF9.CptPw-VyBvFHOKGgRZW670UyP-xO903hJaVxqkHXPKfWCeVn28UGlHLzwKMenbCSIrSF6GQn_Eg2vi9LftqlrQ&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
