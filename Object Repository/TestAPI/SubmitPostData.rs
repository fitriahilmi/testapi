<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SubmitPostData</name>
   <tag></tag>
   <elementGuidId>7708b48a-2164-4c88-a262-97e0329a5768</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${Ltoken}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;lob\&quot; : \&quot;MMU\&quot;,\n    \&quot;fullName\&quot; : \&quot;FITRI\&quot;,\n    \&quot;mobilePhone\&quot; : \&quot;08989898890\&quot;,\n    \&quot;noKtp\&quot; : \&quot;3173082011800008\&quot;,\n    \&quot;birthDate\&quot; : \&quot;1998-11-20\&quot;,\n    \&quot;birthPlace\&quot; : \&quot;JAKARTA\&quot;,\n    \&quot;gender\&quot; : \&quot;M\&quot;,\n    \&quot;mothersName\&quot; : \&quot;CACA\&quot;,\n    \&quot;financingType\&quot; : \&quot;U\&quot;,\n    \&quot;packageCode\&quot; : \&quot;PR0002\&quot;,\n    \&quot;tenor\&quot; : 12,\n    \&quot;downPayment\&quot; : 500000,\n    \&quot;channel\&quot; : \&quot;FNA\&quot;\n}\n&quot;,
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
      <webElementGuid>2d453bd2-f43e-4c8c-a8be-f92d4f6db5b8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${Ltoken}</value>
      <webElementGuid>256d63aa-578f-490b-b02a-7aacd4091f65</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://fifada-qa-lb01.fifgroup.co.id/backend/leads/draft</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>3e657499-3894-40fb-9072-a22572801ea7</id>
      <masked>false</masked>
      <name>Ltoken</name>
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

GlobalVariable.getdrafid = WS.getElementPropertyValue(response, 'result.draftId')
System.out.println(GlobalVariable.getdrafid)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
