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

response1 = WS.sendRequest(findTestObject('LineStart'))

String xml1 = response1.getResponseBodyContent()

def dataValue = new XmlSlurper().parseText(xml1)

def ResultMessage = dataValue.result.Message.text()

def ReqText = ResultMessage.split(':')

print(ResultMessage)

print(ReqText[1])

WebUI.openBrowser("http://a98sv127ixo1d.za.if.atcsg.net/GXOUpgrade_Test/")

WebUI.setText(findTestObject('WebUI/LineStatus/UserName'), "p4035845")

WebUI.click(findTestObject('WebUI/LineStatus/LoginButton'))

WebUI.click(findTestObject('Object Repository/WebUI/LineStatus/Workshop'))

WebUI.click(findTestObject('Object Repository/WebUI/LineStatus/OperOverPage'))

String ReqdTextOnUI = WebUI.getText(findTestObject('Object Repository/WebUI/LineStatus/CurrentLineStatus'))

print(ReqdTextOnUI)